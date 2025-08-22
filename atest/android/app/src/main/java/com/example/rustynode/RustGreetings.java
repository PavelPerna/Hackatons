package com.example.rustynode;

public class RustGreetings {

    private static native String greeting(final String pattern);

    public String sayHello(String to) {
        return greeting(to);
    }
}

static {
    System.loadLibrary("greetings");
}