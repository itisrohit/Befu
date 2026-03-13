package dev.befu.app

import android.annotation.SuppressLint
import android.os.Bundle
import android.util.Log
import android.webkit.WebChromeClient
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import androidx.appcompat.app.AppCompatActivity
import androidx.webkit.WebResourceErrorCompat
import androidx.webkit.WebViewAssetLoader
import androidx.webkit.WebViewClientCompat

private const val TAG = "BefuMainActivity"

class MainActivity : AppCompatActivity() {
    private lateinit var webView: WebView
    private lateinit var assetLoader: WebViewAssetLoader

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        webView = findViewById(R.id.webview)
        with(webView.settings) {
            javaScriptEnabled = true
            domStorageEnabled = true
        }

        assetLoader = WebViewAssetLoader.Builder()
            .addPathHandler(
                "/assets/",
                WebViewAssetLoader.AssetsPathHandler(this),
            )
            .build()

        webView.webChromeClient = WebChromeClient()
        webView.webViewClient = object : WebViewClientCompat() {
            override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest): Boolean {
                val url = request.url
                val allowed = when {
                    url.scheme == "https" &&
                        url.host == "appassets.androidplatform.net" &&
                        (url.path ?: "").startsWith("/assets/") -> true
                    BuildConfig.DEBUG &&
                        url.scheme == "http" &&
                        url.host == "10.0.2.2" &&
                        url.port == 5173 -> true
                    else -> false
                }

                return !allowed
            }

            override fun shouldInterceptRequest(
                view: WebView,
                request: WebResourceRequest,
            ): WebResourceResponse? {
                return assetLoader.shouldInterceptRequest(request.url)
            }

            override fun onReceivedError(
                view: WebView,
                request: WebResourceRequest,
                error: WebResourceErrorCompat,
            ) {
                if (request.isForMainFrame) {
                    Log.e(TAG, "WebView main-frame error: ${error.description} (${error.errorCode})")
                }
            }
        }

        webView.addJavascriptInterface(BefuNativeBridge(), "BefuNative")

        webView.loadUrl(BuildConfig.WEB_ENTRY_URL)
    }

    override fun onDestroy() {
        webView.removeJavascriptInterface("BefuNative")
        webView.destroy()
        super.onDestroy()
    }
}
