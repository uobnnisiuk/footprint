package com.footprint.ble.spike

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.bluetooth.le.AdvertiseCallback
import android.bluetooth.le.AdvertiseData
import android.bluetooth.le.AdvertiseSettings
import android.bluetooth.le.BluetoothLeAdvertiser
import android.bluetooth.le.BluetoothLeScanner
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanFilter
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.os.ParcelUuid
import android.util.Log
import androidx.core.app.NotificationCompat

class BleForegroundService : Service() {
    private var advertiser: BluetoothLeAdvertiser? = null
    private var scanner: BluetoothLeScanner? = null
    private var scanCallback: ScanCallback? = null
    private var advertiseCallback: AdvertiseCallback? = null
    private var started = false

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
        startForeground(BleConstants.NOTIFICATION_ID, buildNotification("BLE active"))
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (!started) {
            started = true
            startBle()
        }
        return START_STICKY
    }

    override fun onDestroy() {
        stopBle()
        stopForeground(STOP_FOREGROUND_REMOVE)
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    private fun startBle() {
        val manager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        val adapter = manager.adapter
        if (adapter == null || !adapter.isEnabled) {
            Log.w(TAG, "Bluetooth disabled or unavailable")
            stopSelf()
            return
        }

        startAdvertising(adapter)
        startScanning(adapter)
    }

    private fun startAdvertising(adapter: BluetoothAdapter) {
        if (!adapter.isMultipleAdvertisementSupported) {
            Log.w(TAG, "BLE advertising not supported")
            return
        }
        advertiser = adapter.bluetoothLeAdvertiser
        if (advertiser == null) {
            Log.w(TAG, "BLE advertiser unavailable")
            return
        }

        val lowPower = Prefs.getLowPowerTag(this) == "low"
        val settings = AdvertiseSettings.Builder()
            .setAdvertiseMode(
                if (lowPower) AdvertiseSettings.ADVERTISE_MODE_LOW_POWER
                else AdvertiseSettings.ADVERTISE_MODE_LOW_LATENCY
            )
            .setConnectable(false)
            .setTimeout(0)
            .setTxPowerLevel(AdvertiseSettings.ADVERTISE_TX_POWER_MEDIUM)
            .build()

        val data = AdvertiseData.Builder()
            .setIncludeDeviceName(false)
            .addServiceUuid(ParcelUuid(BleConstants.SERVICE_UUID))
            .build()

        advertiseCallback = object : AdvertiseCallback() {
            override fun onStartFailure(errorCode: Int) {
                Log.w(TAG, "Advertise failed: $errorCode")
            }

            override fun onStartSuccess(settingsInEffect: AdvertiseSettings) {
                Log.i(TAG, "Advertise started")
            }
        }

        try {
            advertiser?.startAdvertising(settings, data, advertiseCallback)
        } catch (e: SecurityException) {
            Log.w(TAG, "Missing advertise permission", e)
        }
    }

    private fun startScanning(adapter: BluetoothAdapter) {
        scanner = adapter.bluetoothLeScanner
        if (scanner == null) {
            Log.w(TAG, "BLE scanner unavailable")
            return
        }

        val lowPower = Prefs.getLowPowerTag(this) == "low"
        val settings = ScanSettings.Builder()
            .setScanMode(
                if (lowPower) ScanSettings.SCAN_MODE_LOW_POWER
                else ScanSettings.SCAN_MODE_LOW_LATENCY
            )
            .build()

        val filter = ScanFilter.Builder()
            .setServiceUuid(ParcelUuid(BleConstants.SERVICE_UUID))
            .build()

        scanCallback = object : ScanCallback() {
            override fun onScanResult(callbackType: Int, result: ScanResult) {
                handleScanResult(result)
            }

            override fun onBatchScanResults(results: MutableList<ScanResult>) {
                results.forEach { handleScanResult(it) }
            }

            override fun onScanFailed(errorCode: Int) {
                Log.w(TAG, "Scan failed: $errorCode")
            }
        }

        try {
            scanner?.startScan(listOf(filter), settings, scanCallback)
        } catch (e: SecurityException) {
            Log.w(TAG, "Missing scan permission", e)
        }
    }

    private fun handleScanResult(result: ScanResult) {
        BleLogger.logDetection(this, result.rssi)
    }

    private fun stopBle() {
        try {
            scanCallback?.let { scanner?.stopScan(it) }
            advertiseCallback?.let { advertiser?.stopAdvertising(it) }
        } catch (e: SecurityException) {
            Log.w(TAG, "Missing stop permission", e)
        }
        scanCallback = null
        advertiseCallback = null
        scanner = null
        advertiser = null
    }

    private fun buildNotification(content: String): Notification {
        val intent = Intent(this, MainActivity::class.java)
        val pendingIntent = PendingIntent.getActivity(
            this,
            0,
            intent,
            PendingIntent.FLAG_IMMUTABLE
        )
        return NotificationCompat.Builder(this, BleConstants.NOTIFICATION_CHANNEL_ID)
            .setContentTitle("Footprint BLE Spike")
            .setContentText(content)
            .setSmallIcon(android.R.drawable.stat_sys_data_bluetooth)
            .setContentIntent(pendingIntent)
            .setOngoing(true)
            .build()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                BleConstants.NOTIFICATION_CHANNEL_ID,
                "BLE Spike",
                NotificationManager.IMPORTANCE_LOW
            )
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(channel)
        }
    }

    companion object {
        private const val TAG = "BleForegroundService"
    }
}
