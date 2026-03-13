import org.gradle.api.tasks.Exec

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
        release {
            isMinifyEnabled = false
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

    sourceSets {
        getByName("main") {
            jniLibs.srcDir("src/main/jniLibs")
        }
    }
}

val rustOutDir = layout.projectDirectory.dir("src/main/jniLibs")

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

dependencies {
    implementation("androidx.core:core-ktx:1.15.0")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("com.google.android.material:material:1.12.0")
}
