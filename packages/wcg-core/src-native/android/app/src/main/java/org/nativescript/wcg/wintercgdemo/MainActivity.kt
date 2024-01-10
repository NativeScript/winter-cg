package org.nativescript.wcg.wintercgdemo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import org.nativescript.wcg.core.Base64
import org.nativescript.wcg.core.Crypto
import org.nativescript.wcg.core.TextDecoder
import org.nativescript.wcg.core.TextEncoder
import org.nativescript.wcg.core.URL
import java.nio.ByteBuffer
import java.nio.IntBuffer
import java.util.UUID

class MainActivity : AppCompatActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContentView(R.layout.activity_main)
    /*
        val encoder = TextEncoder()
        val decoder = TextDecoder()
        val encoded = encoder.encode("Hello World")
        val encodedBuffer = encoder.encodeBuffer("Hello World")

        val url = URL("../cats", "http://www.example.com/dogs")

        Log.d("com.test", "$url")

        Log.d("com.test", url.hostname); // "www.example.com"
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


        var start = System.currentTimeMillis()
        val encodedBase64 = Base64.btoa("Hello World")

        Log.d("com.test", "btoa ${System.currentTimeMillis() - start}")

        Log.d("com.test", encodedBase64)

        start = System.currentTimeMillis()

        val b64 =
          android.util.Base64.encodeToString("Hello World".toByteArray(), android.util.Base64.DEFAULT)

        Log.d("com.test", "Base64.encodeToString ${System.currentTimeMillis() - start}")

        Log.d("com.test", b64)

        Log.d(
          "com.test",
          "base64 is same ${b64 == encodedBase64} ${b64.length} ${encodedBase64.length}"
        )

        val decodedBase64 = Base64.atob(encodedBase64)

        Log.d("com.test", "decoded ${decodedBase64 == "Hello World"}")


        val charset = java.nio.charset.Charset.forName("UTF-8")

        val d = charset.newDecoder()

        start = System.currentTimeMillis()

        val ddecode = charset.newDecoder().decode(encodedBuffer).toString()

        Log.d("com.test", "newDecoder().decode ${System.currentTimeMillis() - start}")

        start = System.currentTimeMillis()

        val newDecoded = decoder.decode(encodedBuffer)

        Log.d("com.test", "decoder.decode ${System.currentTimeMillis() - start}")



        start = System.currentTimeMillis()

        for (i in 0 until 100) {
          UUID.randomUUID().toString()
        }

        Log.d("com.test", "UUID.randomUUID().toString() ${System.currentTimeMillis() - start}")


        start = System.currentTimeMillis()

        for (i in 0 until 100) {
          Crypto.randomUUID()
        }

        Log.d("com.test", "Crypto.randomUUID() ${System.currentTimeMillis() - start}")

        */

    //val buf = ByteBuffer.allocateDirect(10 * 4)
    val array = IntBuffer.allocate(10 * 4)
    Crypto.getRandomValues(array)

    Log.d("com.test", "Your lucky numbers:")
    val len = array.capacity()
    for (i in 0 until len) {
      Log.d("com.test", "${array.get(i)}")
    }

  }
}
