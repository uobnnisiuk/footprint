package com.footprint.ble.spike

import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.EditText
import android.widget.Spinner
import android.widget.Switch
import android.widget.TextView
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import java.util.UUID

class MainActivity : AppCompatActivity() {
    private lateinit var spinnerDeviceRole: Spinner
    private lateinit var spinnerProximity: Spinner
    private lateinit var spinnerDistance: Spinner
    private lateinit var editDuration: EditText
    private lateinit var switchLowPower: Switch
    private lateinit var textTrialId: TextView

    private var pendingStartService = false

    private val permissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { result ->
        val allGranted = result.values.all { it }
        if (allGranted && pendingStartService) {
            startServiceFlow()
        } else if (!allGranted) {
            pendingStartService = false
            Toast.makeText(this, "Permissions required for BLE", Toast.LENGTH_SHORT).show()
        }
    }

    private val enableBtLauncher = registerForActivityResult(
        ActivityResultContracts.StartActivityForResult()
    ) {
        if (pendingStartService) {
            startServiceFlow()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        spinnerDeviceRole = findViewById(R.id.spinner_device_role)
        spinnerProximity = findViewById(R.id.spinner_proximity)
        spinnerDistance = findViewById(R.id.spinner_distance)
        editDuration = findViewById(R.id.edit_duration)
        switchLowPower = findViewById(R.id.switch_low_power)
        textTrialId = findViewById(R.id.text_trial_id)

        setupSpinner(spinnerDeviceRole, R.array.device_roles, Prefs.getDeviceRole(this))
        setupSpinner(spinnerProximity, R.array.proximity_tags, Prefs.getProximityTag(this))
        setupSpinner(spinnerDistance, R.array.distance_tags, Prefs.getDistanceTag(this))
        editDuration.setText(Prefs.getDurationTarget(this))
        switchLowPower.isChecked = Prefs.getLowPowerTag(this) == "low"
        textTrialId.text = Prefs.getTrialId(this)

        findViewById<Button>(R.id.button_start_trial).setOnClickListener {
            updatePrefsFromUi()
            val trialId = UUID.randomUUID().toString()
            Prefs.setTrialId(this, trialId)
            Prefs.setFirstDetectLogged(this, false)
            textTrialId.text = trialId
            BleLogger.logEvent(this, eventType = "trial_start", appState = "fg", rssi = null)
            Toast.makeText(this, "Trial started", Toast.LENGTH_SHORT).show()
        }

        findViewById<Button>(R.id.button_end_trial).setOnClickListener {
            updatePrefsFromUi()
            BleLogger.logEvent(this, eventType = "trial_end", appState = "fg", rssi = null)
            Toast.makeText(this, "Trial ended", Toast.LENGTH_SHORT).show()
        }

        findViewById<Button>(R.id.button_start_service).setOnClickListener {
            updatePrefsFromUi()
            pendingStartService = true
            if (ensurePermissions()) {
                startServiceFlow()
            }
        }

        findViewById<Button>(R.id.button_stop_service).setOnClickListener {
            pendingStartService = false
            stopService(Intent(this, BleForegroundService::class.java))
        }

        findViewById<Button>(R.id.button_share_logs).setOnClickListener {
            LogShare.shareLog(this)
        }
    }

    private fun setupSpinner(spinner: Spinner, arrayRes: Int, current: String) {
        val items = resources.getStringArray(arrayRes)
        val adapter = ArrayAdapter(this, android.R.layout.simple_spinner_item, items)
        adapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item)
        spinner.adapter = adapter
        val index = items.indexOf(current)
        if (index >= 0) {
            spinner.setSelection(index)
        }
    }

    private fun updatePrefsFromUi() {
        Prefs.setDeviceRole(this, spinnerDeviceRole.selectedItem.toString())
        Prefs.setProximityTag(this, spinnerProximity.selectedItem.toString())
        Prefs.setDistanceTag(this, spinnerDistance.selectedItem.toString())
        Prefs.setDurationTarget(this, editDuration.text.toString().ifBlank { "10" })
        Prefs.setLowPowerTag(this, if (switchLowPower.isChecked) "low" else "normal")
    }

    private fun ensurePermissions(): Boolean {
        val needed = mutableListOf<String>()
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            needed += android.Manifest.permission.BLUETOOTH_SCAN
            needed += android.Manifest.permission.BLUETOOTH_ADVERTISE
            needed += android.Manifest.permission.BLUETOOTH_CONNECT
        } else {
            needed += android.Manifest.permission.ACCESS_FINE_LOCATION
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            needed += android.Manifest.permission.POST_NOTIFICATIONS
        }

        val missing = needed.filter {
            ContextCompat.checkSelfPermission(this, it) !=
                android.content.pm.PackageManager.PERMISSION_GRANTED
        }
        return if (missing.isNotEmpty()) {
            permissionLauncher.launch(missing.toTypedArray())
            false
        } else {
            true
        }
    }

    private fun startServiceFlow() {
        if (!ensureBluetoothEnabled()) {
            return
        }
        pendingStartService = false
        val intent = Intent(this, BleForegroundService::class.java)
        ContextCompat.startForegroundService(this, intent)
        Toast.makeText(this, "Service started", Toast.LENGTH_SHORT).show()
    }

    private fun ensureBluetoothEnabled(): Boolean {
        val manager = getSystemService(BLUETOOTH_SERVICE) as BluetoothManager
        val adapter = manager.adapter
        if (adapter == null) {
            Toast.makeText(this, "Bluetooth not available", Toast.LENGTH_SHORT).show()
            return false
        }
        if (!adapter.isEnabled) {
            enableBtLauncher.launch(Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE))
            return false
        }
        return true
    }
}
