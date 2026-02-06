package com.footprint.ble.spike

import android.app.Activity
import android.content.Intent
import android.widget.Toast
import androidx.core.content.FileProvider
import java.io.File

object LogShare {
    fun shareLog(activity: Activity) {
        val source = File(activity.filesDir, BleLogger.LOG_FILE_NAME)
        if (!source.exists()) {
            Toast.makeText(activity, "Log file not found", Toast.LENGTH_SHORT).show()
            return
        }

        val cacheFile = File(activity.cacheDir, "ble_spike_share.jsonl")
        source.copyTo(cacheFile, overwrite = true)

        val uri = FileProvider.getUriForFile(
            activity,
            "${activity.packageName}.fileprovider",
            cacheFile
        )
        val intent = Intent(Intent.ACTION_SEND).apply {
            type = "application/json"
            putExtra(Intent.EXTRA_STREAM, uri)
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        }
        activity.startActivity(Intent.createChooser(intent, "Share BLE Log"))
    }
}
