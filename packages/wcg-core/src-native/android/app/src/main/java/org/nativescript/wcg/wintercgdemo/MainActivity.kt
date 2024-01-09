package org.nativescript.wcg.wintercgdemo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import org.nativescript.wcg.core.TextDecoder
import org.nativescript.wcg.core.TextEncoder
import org.nativescript.wcg.core.URL

class MainActivity : AppCompatActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)

//    val encoder = TextEncoder()
//    val decoder = TextDecoder()
//    val encoded = encoder.encode("Hello World")
//    val encodedBuffer = encoder.encodeBuffer("Hello World")
//    Log.d("com.test", "decoded ${decoder.decode(encoded)} ${decoder.decode(encodedBuffer)}")

    val url = URL("../cats", "http://www.example.com/dogs")

    Log.d("com.test", "$url")

   // Log.d("com.test", url.hostname); // "www.example.com"
    Log.d("com.test", url.pathname); // "/cats"

    if (URL.canParse("../cats", "http://www.example.com/dogs")) {
      val url = URL("../cats", "http://www.example.com/dogs");
      Log.d("com.test", url.hostname); // "www.example.com"
      Log.d("com.test", url.pathname); // "/cats"
    } else {
      Log.d("com.test", "Invalid URL"); //Invalid URL
    }

    url.hash = "tabby";
    Log.d("com.test", url.href); // "http://www.example.com/cats#tabby"

    url.pathname = "démonstration.html";
    Log.d("com.test", url.href); // "http://www.example.com/d%C3%A9monstration.html"

  }
}
