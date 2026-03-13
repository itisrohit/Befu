package dev.befu.app

import android.annotation.SuppressLint
import android.os.Bundle
import android.util.Log
import android.webkit.WebChromeClient
import android.webkit.WebResourceRequest
import android.webkit.WebResourceError
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.appcompat.app.AppCompatActivity

private const val DEV_SERVER_URL = "http://10.0.2.2:5173"
private const val TAG = "BefuMainActivity"

class MainActivity : AppCompatActivity() {
    private lateinit var webView: WebView

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        webView = findViewById(R.id.webview)
        with(webView.settings) {
            javaScriptEnabled = true
            domStorageEnabled = true
            databaseEnabled = true
        }

        webView.webChromeClient = WebChromeClient()
        webView.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(view: WebView?, request: WebResourceRequest?): Boolean {
                return false
            }

            override fun onReceivedError(
                view: WebView?,
                request: WebResourceRequest?,
                error: WebResourceError?,
            ) {
                if (request?.isForMainFrame == true) {
                    Log.e(TAG, "WebView main-frame error: ${error?.description} (${error?.errorCode})")
                }
            }
        }

        webView.addJavascriptInterface(BefuNativeBridge(), "BefuNative")

        webView.loadUrl(DEV_SERVER_URL)
    }

    override fun onDestroy() {
        webView.removeJavascriptInterface("BefuNative")
        webView.destroy()
        super.onDestroy()
    }
}
