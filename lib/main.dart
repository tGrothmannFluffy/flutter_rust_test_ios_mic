import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:ios_mic/src/rust/api/simple.dart';
import 'package:ios_mic/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  if (kDebugMode) {
    Timer.periodic(const Duration(milliseconds: 15), (Timer t) {
      var message = dequeueDebugMessage();
      if (message != null && message.isNotEmpty) debugPrint(message);
    });
  }
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text('Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
