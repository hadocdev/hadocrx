import 'package:flutter/material.dart';

String _convertToBangla(int number) {
  // Mapping of English digits to Bangla Unicode digits
  const banglaDigits = ['০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯'];

  // Convert the integer to a string to process each digit
  String numberStr = number.toString();

  // Convert each digit to its Bangla equivalent
  String banglaNumber = numberStr.split('').map((char) {
    if (char == '-') return '-'; // Handle negative sign
    int digit = int.parse(char);
    return banglaDigits[digit];
  }).join();

  return banglaNumber;
}

class CounterApp extends StatelessWidget {
  const CounterApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
          colorScheme: ColorScheme.dark(),
          useMaterial3: true,
          fontFamily: 'SolaimanLipi'),
      home: const CounterHomePage(title: 'কাউন্টার'),
      debugShowCheckedModeBanner: false,
    );
  }
}

class CounterHomePage extends StatefulWidget {
  const CounterHomePage({super.key, required this.title});
  final String title;

  @override
  State<CounterHomePage> createState() => _CounterHomePageState();
}

class _CounterHomePageState extends State<CounterHomePage> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'তুই ${_convertToBangla(_counter)} বার কিলিক মারছোদ',
              style: TextStyle(
                fontFamily: 'SolaimanLipi',
                fontSize: 32,
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ), // This trailing comma makes auto-formatting nicer for build methods.
    );
  }
}
