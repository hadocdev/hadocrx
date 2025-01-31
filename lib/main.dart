import 'package:flutter/material.dart';
import 'package:avro_phonetic_textfield/avro_phonetic_textfield.dart';
// import 'package:hadocrx/avro_phonetic_textfield.dart';

void main() {
  runApp(const AvroPhoneticApp());
}

class AvroPhoneticApp extends StatefulWidget {
  const AvroPhoneticApp({super.key});

  @override
  State<AvroPhoneticApp> createState() => _AvroPhoneticAppState();
}

class _AvroPhoneticAppState extends State<AvroPhoneticApp> {
  String _name = '';
  final TextEditingController _nameController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
          colorScheme: ColorScheme.dark(),
          useMaterial3: true,
          fontFamily: 'SolaimanLipi'),
      home: Scaffold(
        body: Padding(
          padding: EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(_name),
              SizedBox(height: 20),
              AvroPhoneticTextField(
                controller: _nameController,
                maxLines: 5,
                autofocus: true,
                decoration: InputDecoration(
                    labelText: 'Name', border: OutlineInputBorder()),
              ),
              SizedBox(
                height: 20,
              ),
              FilledButton(
                  onPressed: () {
                    setState(() {
                      _name = _nameController.text;
                    });
                  },
                  child: Text('Submit'))
            ],
          ),
        ),
      ),
      debugShowCheckedModeBanner: false,
    );
  }
}
