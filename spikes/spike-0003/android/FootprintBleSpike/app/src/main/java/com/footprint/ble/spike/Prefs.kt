package com.footprint.ble.spike

import android.content.Context
import android.content.SharedPreferences
import java.util.UUID

object Prefs {
    private const val NAME = "ble_spike_prefs"

    private const val KEY_PEER_ID = "peer_id"
    private const val KEY_TRIAL_ID = "trial_id"
    private const val KEY_DEVICE_ROLE = "device_role"
    private const val KEY_PROXIMITY_TAG = "proximity_tag"
    private const val KEY_DISTANCE_TAG = "distance_tag"
    private const val KEY_DURATION_TARGET = "duration_target_min"
    private const val KEY_LOW_POWER = "low_power_tag"
    private const val KEY_FIRST_DETECT = "first_detect_logged"

    private fun prefs(context: Context): SharedPreferences {
        return context.getSharedPreferences(NAME, Context.MODE_PRIVATE)
    }

    fun getPeerId(context: Context): String {
        val p = prefs(context)
        val existing = p.getString(KEY_PEER_ID, null)
        if (existing != null && existing.isNotBlank()) {
            return existing
        }
        val generated = UUID.randomUUID().toString()
        p.edit().putString(KEY_PEER_ID, generated).apply()
        return generated
    }

    fun setTrialId(context: Context, trialId: String) {
        prefs(context).edit().putString(KEY_TRIAL_ID, trialId).apply()
    }

    fun getTrialId(context: Context): String {
        return prefs(context).getString(KEY_TRIAL_ID, "no_trial") ?: "no_trial"
    }

    fun setDeviceRole(context: Context, role: String) {
        prefs(context).edit().putString(KEY_DEVICE_ROLE, role).apply()
    }

    fun getDeviceRole(context: Context): String {
        return prefs(context).getString(KEY_DEVICE_ROLE, "alpha") ?: "alpha"
    }

    fun setProximityTag(context: Context, tag: String) {
        prefs(context).edit().putString(KEY_PROXIMITY_TAG, tag).apply()
    }

    fun getProximityTag(context: Context): String {
        return prefs(context).getString(KEY_PROXIMITY_TAG, "near") ?: "near"
    }

    fun setDistanceTag(context: Context, tag: String) {
        prefs(context).edit().putString(KEY_DISTANCE_TAG, tag).apply()
    }

    fun getDistanceTag(context: Context): String {
        return prefs(context).getString(KEY_DISTANCE_TAG, "1") ?: "1"
    }

    fun setDurationTarget(context: Context, value: String) {
        prefs(context).edit().putString(KEY_DURATION_TARGET, value).apply()
    }

    fun getDurationTarget(context: Context): String {
        return prefs(context).getString(KEY_DURATION_TARGET, "10") ?: "10"
    }

    fun setLowPowerTag(context: Context, value: String) {
        prefs(context).edit().putString(KEY_LOW_POWER, value).apply()
    }

    fun getLowPowerTag(context: Context): String {
        return prefs(context).getString(KEY_LOW_POWER, "normal") ?: "normal"
    }

    fun setFirstDetectLogged(context: Context, value: Boolean) {
        prefs(context).edit().putBoolean(KEY_FIRST_DETECT, value).apply()
    }

    fun isFirstDetectLogged(context: Context): Boolean {
        return prefs(context).getBoolean(KEY_FIRST_DETECT, false)
    }
}
