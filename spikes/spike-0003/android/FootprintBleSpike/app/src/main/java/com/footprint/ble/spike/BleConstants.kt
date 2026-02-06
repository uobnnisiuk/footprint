package com.footprint.ble.spike

import java.util.UUID

object BleConstants {
    val SERVICE_UUID: UUID = UUID.fromString("0000fdde-0000-1000-8000-00805f9b34fb")
    const val NOTIFICATION_CHANNEL_ID = "ble_spike_channel"
    const val NOTIFICATION_ID = 1103
}
