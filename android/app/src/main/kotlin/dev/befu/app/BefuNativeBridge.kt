package dev.befu.app

import android.os.Build
import android.webkit.JavascriptInterface
import org.json.JSONObject

class BefuNativeBridge {
    @JavascriptInterface
    fun backendMode(): String {
        return if (isNativeLoaded) "jni" else "fallback"
    }

    @JavascriptInterface
    fun invokeRaw(payloadJson: String): String {
        return runCatching {
            if (isNativeLoaded) {
                return@runCatching invokeRawNative(payloadJson)
            }

            fallbackInvokeRaw(payloadJson)
        }.getOrElse { error ->
            JSONObject()
                .put("id", "")
                .put("ok", false)
                .put(
                    "error",
                    JSONObject()
                        .put("code", "NATIVE_BRIDGE_FAILURE")
                        .put("message", error.message ?: "Unknown native bridge failure"),
                )
                .toString()
        }
    }

    private fun fallbackInvokeRaw(payloadJson: String): String {
        val request = JSONObject(payloadJson)
        val id = request.optString("id")
        val command = request.optString("command")

        return when (command) {
            "ping" -> JSONObject()
                .put("id", id)
                .put("ok", true)
                .put("result", JSONObject().put("pong", "pong"))
                .toString()

            "app.info" -> JSONObject()
                .put("id", id)
                .put("ok", true)
                .put(
                    "result",
                    JSONObject()
                        .put("name", "Befu")
                        .put("version", BuildConfig.VERSION_NAME)
                        .put("runtime", "befu"),
                )
                .toString()

            else -> JSONObject()
                .put("id", id)
                .put("ok", false)
                .put(
                    "error",
                    JSONObject()
                        .put("code", "UNKNOWN_COMMAND")
                        .put("message", "Unknown command: $command"),
                )
                .toString()
        }
    }

    companion object {
        private var isNativeLoaded = false

        init {
            isNativeLoaded = runCatching {
                System.loadLibrary("befu_core")
                true
            }.getOrDefault(false)
        }

        @JvmStatic
        private external fun invokeRawNative(payloadJson: String): String
    }
}
