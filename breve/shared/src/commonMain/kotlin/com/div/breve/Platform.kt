package com.div.breve

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform