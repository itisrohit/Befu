import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.Sync

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "dev.befu.app"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.befu.app"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
        }
    }

    buildTypes {
        debug {
            signingConfig = signingConfigs.getByName("debug")
            buildConfigField("String", "WEB_ENTRY_URL", "\"http://10.0.2.2:5173\"")
        }

        release {
            isMinifyEnabled = false
            signingConfig = signingConfigs.getByName("debug")
            buildConfigField(
                "String",
                "WEB_ENTRY_URL",
                "\"https://appassets.androidplatform.net/assets/index.html\""
            )
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        buildConfig = true
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDir("src/main/jniLibs")
            assets.setSrcDirs(listOf(layout.buildDirectory.dir("generated/befu-web-assets")))
        }
    }
}

val rustOutDir = layout.projectDirectory.dir("src/main/jniLibs")
val webAssetsDir = layout.buildDirectory.dir("generated/befu-web-assets")
val webDistDir = rootProject.projectDir.parentFile.resolve("apps/web/dist")

val buildRustAndroidLibs by tasks.registering(Exec::class) {
    group = "build"
    description = "Builds befu-core Rust Android shared libraries"

    workingDir = rootProject.projectDir.parentFile
    commandLine(
        "cargo",
        "ndk",
        "-t",
        "armeabi-v7a",
        "-t",
        "arm64-v8a",
        "-t",
        "x86_64",
        "-o",
        rustOutDir.asFile.absolutePath,
        "build",
        "--manifest-path",
        "crates/core/Cargo.toml",
        "--release"
    )
}

tasks.matching { it.name.startsWith("merge") && it.name.endsWith("JniLibFolders") }
    .configureEach {
        dependsOn(buildRustAndroidLibs)
    }

val buildWebAssets by tasks.registering(Exec::class) {
    group = "build"
    description = "Builds web assets for Android release"

    workingDir = rootProject.projectDir.parentFile
    commandLine("bun", "run", "build")
}

val syncWebAssets by tasks.registering(Sync::class) {
    group = "build"
    description = "Syncs built web assets into Android assets directory"

    dependsOn(buildWebAssets)
    from(webDistDir)
    into(webAssetsDir)
}

tasks.matching {
    it.name == "mergeDebugAssets" ||
    it.name == "mergeReleaseAssets" ||
    it.name == "generateReleaseLintVitalReportModel" ||
        it.name == "lintVitalAnalyzeRelease" ||
        it.name == "lintVitalReportRelease" ||
        it.name == "lintVitalRelease"
}
    .configureEach {
        dependsOn(syncWebAssets)
    }

dependencies {
    implementation("androidx.core:core-ktx:1.15.0")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.webkit:webkit:1.13.0")
}
