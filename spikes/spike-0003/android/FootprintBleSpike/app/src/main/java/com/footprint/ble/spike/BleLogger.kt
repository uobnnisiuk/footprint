package com.footprint.ble.spike

import android.content.Context
import android.os.BatteryManager
import org.json.JSONObject
import java.io.File
import java.io.FileOutputStream
import java.nio.charset.StandardCharsets
import java.time.Instant

object BleLogger {
    const val LOG_FILE_NAME = "ble_spike.jsonl"
    private val lock = Any()

    fun logDetection(context: Context, rssi: Int) {
        val firstLogged = Prefs.isFirstDetectLogged(context)
        if (!firstLogged) {
            logEvent(context, eventType = "first_detect", appState = "service", rssi = rssi)
            Prefs.setFirstDetectLogged(context, true)
        }
        logEvent(context, eventType = "detect", appState = "service", rssi = rssi)
    }

    fun logEvent(
        context: Context,
        eventType: String,
        appState: String,
        rssi: Int?
    ) {
        val json = JSONObject()
        json.put("ts", Instant.now().toString())
        json.put("trial_id", Prefs.getTrialId(context))
        json.put("device_role", Prefs.getDeviceRole(context))
        json.put("os", "Android")
        json.put("app_state", appState)
        json.put("low_power_tag", Prefs.getLowPowerTag(context))
        json.put("proximity_tag", Prefs.getProximityTag(context))
        json.put("distance_m_tag", Prefs.getDistanceTag(context))
        putDuration(json, Prefs.getDurationTarget(context))
        json.put("peer_id", Prefs.getPeerId(context))
        json.put("event_type", eventType)
        if (rssi == null) {
            json.put("rssi", JSONObject.NULL)
        } else {
            json.put("rssi", rssi)
        }
        json.put("battery_pct", readBatteryPct(context))

        val line = json.toString() + "\n"
        val file = File(context.filesDir, LOG_FILE_NAME)
        synchronized(lock) {
            FileOutputStream(file, true).use { stream ->
                stream.write(line.toByteArray(StandardCharsets.UTF_8))
            }
        }
    }

    private fun putDuration(json: JSONObject, value: String) {
        val trimmed = value.trim()
        val numeric = trimmed.toDoubleOrNull()
        if (numeric == null) {
            json.put("duration_target_min", trimmed)
        } else {
            json.put("duration_target_min", numeric)
        }
    }

    private fun readBatteryPct(context: Context): Int {
        val manager = context.getSystemService(Context.BATTERY_SERVICE) as BatteryManager
        val pct = manager.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY)
        return pct
    }
}
