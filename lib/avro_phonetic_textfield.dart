import 'dart:convert';
import 'dart:ui';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class AvroPhoneticTextField extends StatefulWidget {
  final TextEditingController controller;
  final FocusNode? focusNode;
  final InputDecoration? decoration;
  final TextInputType? keyboardType;
  final TextInputAction? textInputAction;
  final TextCapitalization? textCapitalization;
  final TextStyle? style;
  final StrutStyle? strutStyle;
  final TextAlign? textAlign;
  final TextAlignVertical? textAlignVertical;
  final TextDirection? textDirection;
  final bool? readOnly;
  final bool? showCursor;
  final bool? autofocus;
  final String? obscuringCharacter;
  final bool? obscureText;
  final bool? autocorrect;
  final SmartDashesType? smartDashesType;
  final SmartQuotesType? smartQuotesType;
  final bool? enableSuggestions;
  final int? maxLines;
  final int? minLines;
  final bool? expands;
  final int? maxLength;
  final MaxLengthEnforcement? maxLengthEnforcement;
  final ValueChanged<String>? onChanged;
  final VoidCallback? onEditingComplete;
  final ValueChanged<String>? onSubmitted;
  final AppPrivateCommandCallback? onAppPrivateCommand;
  final List<TextInputFormatter>? inputFormatters;
  final bool? enabled;
  final double? cursorWidth;
  final double? cursorHeight;
  final Radius? cursorRadius;
  final bool? cursorOpacityAnimates;
  final Color? cursorColor;
  final Color? cursorErrorColor;
  final BoxHeightStyle? selectionHeightStyle;
  final BoxWidthStyle? selectionWidthStyle;
  final Brightness? keyboardAppearance;
  final EdgeInsets? scrollPadding;
  final DragStartBehavior? dragStartBehavior;
  final bool? enableInteractiveSelection;
  final TextSelectionControls? selectionControls;
  final GestureTapCallback? onTap;
  final bool? onTapAlwaysCalled;
  final TapRegionCallback? onTapOutside;
  final MouseCursor? mouseCursor;
  final InputCounterWidgetBuilder? buildCounter;
  final ScrollController? scrollController;
  final ScrollPhysics? scrollPhysics;
  final Iterable<String>? autofillHints;
  final ContentInsertionConfiguration? contentInsertionConfiguration;
  final Clip? clipBehavior;
  final String? restorationId;
  final bool? scribbleEnabled;
  final bool? enableIMEPersonalizedLearning;
  final EditableTextContextMenuBuilder? contextMenuBuilder;
  final bool? canRequestFocus;
  final SpellCheckConfiguration? spellCheckConfiguration;
  final TextMagnifierConfiguration? magnifierConfiguration;

  static Widget _defaultContextMenuBuilder(
      BuildContext context, EditableTextState editableTextState) {
    return AdaptiveTextSelectionToolbar.editableText(
      editableTextState: editableTextState,
    );
  }

  const AvroPhoneticTextField({
    super.key,
    required this.controller,
    this.focusNode,
    this.decoration,
    this.keyboardType,
    this.textInputAction,
    this.textCapitalization,
    this.style,
    this.strutStyle,
    this.textAlign,
    this.textAlignVertical,
    this.textDirection,
    this.readOnly,
    this.showCursor,
    this.autofocus,
    this.obscuringCharacter,
    this.obscureText,
    this.autocorrect,
    this.smartDashesType,
    this.smartQuotesType,
    this.enableSuggestions,
    this.maxLines,
    this.minLines,
    this.expands,
    this.maxLength,
    this.maxLengthEnforcement,
    this.onChanged,
    this.onEditingComplete,
    this.onSubmitted,
    this.onAppPrivateCommand,
    this.inputFormatters,
    this.enabled,
    this.cursorWidth,
    this.cursorHeight,
    this.cursorRadius,
    this.cursorOpacityAnimates,
    this.cursorColor,
    this.cursorErrorColor,
    this.selectionHeightStyle,
    this.selectionWidthStyle,
    this.keyboardAppearance,
    this.scrollPadding,
    this.dragStartBehavior,
    this.enableInteractiveSelection,
    this.selectionControls,
    this.onTap,
    this.onTapAlwaysCalled,
    this.onTapOutside,
    this.mouseCursor,
    this.buildCounter,
    this.scrollController,
    this.scrollPhysics,
    this.autofillHints,
    this.contentInsertionConfiguration,
    this.clipBehavior,
    this.restorationId,
    this.scribbleEnabled,
    this.enableIMEPersonalizedLearning,
    this.contextMenuBuilder,
    this.canRequestFocus,
    this.spellCheckConfiguration,
    this.magnifierConfiguration,
  });

  @override
  State<AvroPhoneticTextField> createState() => _AvroPhoneticTextFieldState();
}

class _AvroPhoneticTextFieldState extends State<AvroPhoneticTextField> {
  String _total = '';
  String _current = '';
  bool _bangla = true;
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    widget.controller.addListener(() {
      String temp = widget.controller.text;
      bool textAdded = (temp.length == _total.length + 1);
      bool textDeleted = (temp.length == _total.length - 1);
      if (_bangla && !HardwareKeyboard.instance.isControlPressed) {
        if (textAdded) {
          String lastChar = temp[temp.length - 1];
          if (lastChar == ' ') {
            WidgetsBinding.instance.addPostFrameCallback((_) {
              setState(() {
                String newText =
                    _replaceFirstFromEnd(_total, _current, parse(_current));
                widget.controller.value = TextEditingValue(
                    text: newText,
                    selection: TextSelection.collapsed(offset: newText.length));
                _current = '';
              });
            });
          } else {
            setState(() {
              _current += lastChar;
            });
          }
        }
        if (textDeleted) {
          setState(() {
            _current = _current.isNotEmpty
                ? _current.substring(0, _current.length - 1)
                : _current;
          });
        }
      }
      setState(() {
        _total = widget.controller.text;
      });
    });
  }

  @override
  void dispose() {
    super.dispose();
    widget.controller.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        KeyboardListener(
          focusNode: _focusNode,
          onKeyEvent: (event) {
            if (event is KeyDownEvent &&
                HardwareKeyboard.instance.isControlPressed &&
                event.logicalKey.keyLabel == 'M') {
              setState(() {
                _bangla = !_bangla;
                _current = '';
              });
            }
          },
          child: TextField(
            controller: widget.controller,
            focusNode: widget.focusNode,
            decoration: widget.decoration ?? const InputDecoration(),
            keyboardType: widget.keyboardType,
            textInputAction: widget.textInputAction,
            textCapitalization:
                widget.textCapitalization ?? TextCapitalization.none,
            style: widget.style,
            strutStyle: widget.strutStyle,
            textAlign: widget.textAlign ?? TextAlign.start,
            textAlignVertical: widget.textAlignVertical,
            textDirection: widget.textDirection,
            readOnly: widget.readOnly ?? false,
            showCursor: widget.showCursor,
            autofocus: widget.autofocus ?? false,
            obscuringCharacter: widget.obscuringCharacter ?? '•',
            obscureText: widget.obscureText ?? false,
            autocorrect: widget.autocorrect ?? true,
            smartDashesType: widget.smartDashesType,
            smartQuotesType: widget.smartQuotesType,
            enableSuggestions: widget.enableSuggestions ?? true,
            maxLines: widget.maxLines ?? 1,
            minLines: widget.minLines,
            expands: widget.expands ?? false,
            maxLength: widget.maxLength,
            maxLengthEnforcement: widget.maxLengthEnforcement,
            onChanged: widget.onChanged,
            onEditingComplete: widget.onEditingComplete,
            onSubmitted: widget.onSubmitted,
            onAppPrivateCommand: widget.onAppPrivateCommand,
            inputFormatters: widget.inputFormatters,
            enabled: widget.enabled,
            cursorWidth: widget.cursorWidth ?? 2.0,
            cursorHeight: widget.cursorHeight,
            cursorRadius: widget.cursorRadius,
            cursorOpacityAnimates: widget.cursorOpacityAnimates,
            cursorColor: widget.cursorColor,
            cursorErrorColor: widget.cursorErrorColor,
            selectionHeightStyle:
                widget.selectionHeightStyle ?? BoxHeightStyle.tight,
            selectionWidthStyle:
                widget.selectionWidthStyle ?? BoxWidthStyle.tight,
            keyboardAppearance: widget.keyboardAppearance,
            scrollPadding: widget.scrollPadding ?? const EdgeInsets.all(20.0),
            dragStartBehavior:
                widget.dragStartBehavior ?? DragStartBehavior.start,
            enableInteractiveSelection:
                widget.enableInteractiveSelection ?? true,
            selectionControls: widget.selectionControls,
            onTap: widget.onTap,
            onTapAlwaysCalled: widget.onTapAlwaysCalled ?? false,
            onTapOutside: widget.onTapOutside,
            mouseCursor: widget.mouseCursor,
            buildCounter: widget.buildCounter,
            scrollController: widget.scrollController,
            scrollPhysics: widget.scrollPhysics,
            autofillHints: widget.autofillHints,
            contentInsertionConfiguration: widget.contentInsertionConfiguration,
            clipBehavior: widget.clipBehavior ?? Clip.hardEdge,
            restorationId: widget.restorationId,
            scribbleEnabled: widget.scribbleEnabled ?? true,
            enableIMEPersonalizedLearning:
                widget.enableIMEPersonalizedLearning ?? true,
            contextMenuBuilder: widget.contextMenuBuilder ??
                AvroPhoneticTextField._defaultContextMenuBuilder,
            canRequestFocus: widget.canRequestFocus ?? true,
            spellCheckConfiguration: widget.spellCheckConfiguration,
            magnifierConfiguration: widget.magnifierConfiguration,
          ),
        )
      ],
    );
  }
}

String _replaceFirstFromEnd(
    String input, String substring, String replacement) {
  // Reverse the input and substring
  String reversedInput = input.split('').reversed.join();
  String reversedSubstring = substring.split('').reversed.join();
  String reversedReplacement = replacement.split('').reversed.join();

  // Replace the first occurrence of the reversed substring
  String result =
      reversedInput.replaceFirst(reversedSubstring, reversedReplacement);

  // Reverse the result back
  result = result.split('').reversed.join();
  return result;
}

final _data = jsonDecode(jsonData);

String fixString(String input) {
  String fixed = '';
  for (int i = 0; i < input.length; ++i) {
    String cChar = input[i];
    if (isCaseSensitive(cChar)) {
      fixed += cChar;
    } else {
      fixed += cChar.toLowerCase();
    }
  }
  return fixed;
}

bool isVowel(String c) {
  return _data['vowel']!.contains(c.toLowerCase());
}

bool isConsonant(String c) {
  return _data['consonant']!.contains(c.toLowerCase());
}

bool isPunctuation(String c) {
  return !(isVowel(c) || isConsonant(c));
}

bool isExact(String needle, String haystack, int start, int end, bool not) {
  return (start >= 0 &&
          end < haystack.length &&
          haystack.substring(start, end) == needle) ^
      not;
}

bool isCaseSensitive(String c) {
  return _data['casesensitive']!.contains(c.toLowerCase());
}

String parse(String input) {
  String fixed = fixString(input);
  String output = "";

  for (int cur = 0; cur < fixed.length; ++cur) {
    int start = cur, end = cur + 1, prev = start - 1;
    bool matched = false;

    for (var pattern in _data['patterns']) {
      String find = pattern['find'];
      end = cur + find.length;

      if (end <= fixed.length &&
          fixed.substring(start, end) == pattern['find']) {
        prev = start - 1;

        if (pattern.containsKey('rules')) {
          for (var rule in pattern['rules']) {
            bool replace = true;

            for (var match in rule['matches']) {
              int chk = 0;
              if (match['type'] == 'suffix') {
                chk = end;
              } else {
                chk = prev;
              }

              // Handle Negative
              if (!match.containsKey('negative')) {
                match['negative'] = false;
                if (match['scope'].startsWith('!')) {
                  match['negative'] = true;
                  match['scope'] = match['scope'].substring(1);
                }
              }

              // Handle empty value
              match['value'] ??= '';

              // Beginning
              if (match['scope'] == 'punctuation') {
                if (!(((chk < 0 && match['type'] == 'prefix') ||
                        (chk >= fixed.length && match['type'] == 'suffix') ||
                        isPunctuation(fixed[chk])) ^
                    match['negative'])) {
                  replace = false;
                  break;
                }
              }
              // Vowel
              else if (match['scope'] == 'vowel') {
                if (!(((chk >= 0 && match['type'] == 'prefix') ||
                            (chk < fixed.length &&
                                match['type'] == 'suffix')) &&
                        isVowel(fixed[chk])) ^
                    match['negative']) {
                  replace = false;
                  break;
                }
              }
              // Consonant
              else if (match['scope'] == 'consonant') {
                if (!(((chk >= 0 && match['type'] == 'prefix') ||
                            (chk < fixed.length &&
                                match['type'] == 'suffix')) &&
                        isConsonant(fixed[chk])) ^
                    match['negative']) {
                  replace = false;
                  break;
                }
              }
              // Exact
              else if (match['scope'] == 'exact') {
                int s, e;
                if (match['type'] == 'suffix') {
                  s = end;
                  String value = match['value'];
                  e = end + value.length;
                } else {
                  String value = match['value'];
                  s = start - value.length;
                  e = start;
                }
                if (!isExact(match['value'], fixed, s, e, match['negative'])) {
                  replace = false;
                  break;
                }
              }
            }

            if (replace) {
              output += rule['replace'];
              cur = end - 1;
              matched = true;
              break;
            }
          }
        }

        if (matched) break;

        // Default
        output += pattern['replace'];
        cur = end - 1;
        matched = true;
        break;
      }
    }

    if (!matched) {
      output += fixed[cur];
    }
  }

  return output;
}

final String jsonData = '''{
        "patterns":
        [
            {
                "find":"bhl",
                "replace":"ভ্ল"
            },
            {
                "find":"psh",
                "replace":"পশ"
            },
            {
                "find":"bdh",
                "replace":"ব্ধ"
            },
            {
                "find":"bj",
                "replace":"ব্জ"
            },
            {
                "find":"bd",
                "replace":"ব্দ"
            },
            {
                "find":"bb",
                "replace":"ব্ব"
            },
            {
                "find":"bl",
                "replace":"ব্ল"
            },
            {
                "find":"bh",
                "replace":"ভ"
            },
            {
                "find":"vl",
                "replace":"ভ্ল"
            },
            {
                "find":"b",
                "replace":"ব"
            },
            {
                "find":"v",
                "replace":"ভ"
            },
            {
                "find":"cNG",
                "replace":"চ্ঞ"
            },
            {
                "find":"cch",
                "replace":"চ্ছ"
            },
            {
                "find":"cc",
                "replace":"চ্চ"
            },
            {
                "find":"ch",
                "replace":"ছ"
            },
            {
                "find":"c",
                "replace":"চ"
            },
            {
                "find":"dhn",
                "replace":"ধ্ন"
            },
            {
                "find":"dhm",
                "replace":"ধ্ম"
            },
            {
                "find":"dgh",
                "replace":"দ্ঘ"
            },
            {
                "find":"ddh",
                "replace":"দ্ধ"
            },
            {
                "find":"dbh",
                "replace":"দ্ভ"
            },
            {
                "find":"dv",
                "replace":"দ্ভ"
            },
            {
                "find":"dm",
                "replace":"দ্ম"
            },
            {
                "find":"DD",
                "replace":"ড্ড"
            },
            {
                "find":"Dh",
                "replace":"ঢ"
            },
            {
                "find":"dh",
                "replace":"ধ"
            },
            {
                "find":"dg",
                "replace":"দ্গ"
            },
            {
                "find":"dd",
                "replace":"দ্দ"
            },
            {
                "find":"D",
                "replace":"ড"
            },
            {
                "find":"d",
                "replace":"দ"
            },
            {
                "find":"...",
                "replace":"..."
            },
            {
                "find":".`",
                "replace":"."
            },
            {
                "find":"..",
                "replace":"।।"
            },
            {
                "find":".",
                "replace":"।"
            },
            {
                "find":"ghn",
                "replace":"ঘ্ন"
            },
            {
                "find":"Ghn",
                "replace":"ঘ্ন"
            },
            {
                "find":"gdh",
                "replace":"গ্ধ"
            },
            {
                "find":"Gdh",
                "replace":"গ্ধ"
            },
            {
                "find":"gN",
                "replace":"গ্ণ"
            },
            {
                "find":"GN",
                "replace":"গ্ণ"
            },
            {
                "find":"gn",
                "replace":"গ্ন"
            },
            {
                "find":"Gn",
                "replace":"গ্ন"
            },
            {
                "find":"gm",
                "replace":"গ্ম"
            },
            {
                "find":"Gm",
                "replace":"গ্ম"
            },
            {
                "find":"gl",
                "replace":"গ্ল"
            },
            {
                "find":"Gl",
                "replace":"গ্ল"
            },
            {
                "find":"gg",
                "replace":"জ্ঞ"
            },
            {
                "find":"GG",
                "replace":"জ্ঞ"
            },
            {
                "find":"Gg",
                "replace":"জ্ঞ"
            },
            {
                "find":"gG",
                "replace":"জ্ঞ"
            },
            {
                "find":"gh",
                "replace":"ঘ"
            },
            {
                "find":"Gh",
                "replace":"ঘ"
            },
            {
                "find":"g",
                "replace":"গ"
            },
            {
                "find":"G",
                "replace":"গ"
            },
            {
                "find":"hN",
                "replace":"হ্ণ"
            },
            {
                "find":"hn",
                "replace":"হ্ন"
            },
            {
                "find":"hm",
                "replace":"হ্ম"
            },
            {
                "find":"hl",
                "replace":"হ্ল"
            },
            {
                "find":"h",
                "replace":"হ"
            },
            {
                "find":"jjh",
                "replace":"জ্ঝ"
            },
            {
                "find":"jNG",
                "replace":"জ্ঞ"
            },
            {
                "find":"jh",
                "replace":"ঝ"
            },
            {
                "find":"jj",
                "replace":"জ্জ"
            },
            {
                "find":"j",
                "replace":"জ"
            },
            {
                "find":"J",
                "replace":"জ"
            },
            {
                "find":"kkhN",
                "replace":"ক্ষ্ণ"
            },
            {
                "find":"kShN",
                "replace":"ক্ষ্ণ"
            },
            {
                "find":"kkhm",
                "replace":"ক্ষ্ম"
            },
            {
                "find":"kShm",
                "replace":"ক্ষ্ম"
            },
            {
                "find":"kxN",
                "replace":"ক্ষ্ণ"
            },
            {
                "find":"kxm",
                "replace":"ক্ষ্ম"
            },
            {
                "find":"kkh",
                "replace":"ক্ষ"
            },
            {
                "find":"kSh",
                "replace":"ক্ষ"
            },
            {
                "find":"ksh",
                "replace":"কশ"
            },
            {
                "find":"kx",
                "replace":"ক্ষ"
            },
            {
                "find":"kk",
                "replace":"ক্ক"
            },
            {
                "find":"kT",
                "replace":"ক্ট"
            },
            {
                "find":"kt",
                "replace":"ক্ত"
            },
            {
                "find":"kl",
                "replace":"ক্ল"
            },
            {
                "find":"ks",
                "replace":"ক্স"
            },
            {
                "find":"kh",
                "replace":"খ"
            },
            {
                "find":"k",
                "replace":"ক"
            },
            {
                "find":"lbh",
                "replace":"ল্ভ"
            },
            {
                "find":"ldh",
                "replace":"ল্ধ"
            },
            {
                "find":"lkh",
                "replace":"লখ"
            },
            {
                "find":"lgh",
                "replace":"লঘ"
            },
            {
                "find":"lph",
                "replace":"লফ"
            },
            {
                "find":"lk",
                "replace":"ল্ক"
            },
            {
                "find":"lg",
                "replace":"ল্গ"
            },
            {
                "find":"lT",
                "replace":"ল্ট"
            },
            {
                "find":"lD",
                "replace":"ল্ড"
            },
            {
                "find":"lp",
                "replace":"ল্প"
            },
            {
                "find":"lv",
                "replace":"ল্ভ"
            },
            {
                "find":"lm",
                "replace":"ল্ম"
            },
            {
                "find":"ll",
                "replace":"ল্ল"
            },
            {
                "find":"lb",
                "replace":"ল্ব"
            },
            {
                "find":"l",
                "replace":"ল"
            },
            {
                "find":"mth",
                "replace":"ম্থ"
            },
            {
                "find":"mph",
                "replace":"ম্ফ"
            },
            {
                "find":"mbh",
                "replace":"ম্ভ"
            },
            {
                "find":"mpl",
                "replace":"মপ্ল"
            },
            {
                "find":"mn",
                "replace":"ম্ন"
            },
            {
                "find":"mp",
                "replace":"ম্প"
            },
            {
                "find":"mv",
                "replace":"ম্ভ"
            },
            {
                "find":"mm",
                "replace":"ম্ম"
            },
            {
                "find":"ml",
                "replace":"ম্ল"
            },
            {
                "find":"mb",
                "replace":"ম্ব"
            },
            {
                "find":"mf",
                "replace":"ম্ফ"
            },
            {
                "find":"m",
                "replace":"ম"
            },
            {
                "find":"0",
                "replace":"০"
            },
            {
                "find":"1",
                "replace":"১"
            },
            {
                "find":"2",
                "replace":"২"
            },
            {
                "find":"3",
                "replace":"৩"
            },
            {
                "find":"4",
                "replace":"৪"
            },
            {
                "find":"5",
                "replace":"৫"
            },
            {
                "find":"6",
                "replace":"৬"
            },
            {
                "find":"7",
                "replace":"৭"
            },
            {
                "find":"8",
                "replace":"৮"
            },
            {
                "find":"9",
                "replace":"৯"
            },
            {
                "find":"NgkSh",
                "replace":"ঙ্ক্ষ"
            },
            {
                "find":"Ngkkh",
                "replace":"ঙ্ক্ষ"
            },
            {
                "find":"NGch",
                "replace":"ঞ্ছ"
            },
            {
                "find":"Nggh",
                "replace":"ঙ্ঘ"
            },
            {
                "find":"Ngkh",
                "replace":"ঙ্খ"
            },
            {
                "find":"NGjh",
                "replace":"ঞ্ঝ"
            },
            {
                "find":"ngOU",
                "replace":"ঙ্গৌ"
            },
            {
                "find":"ngOI",
                "replace":"ঙ্গৈ"
            },
            {
                "find":"Ngkx",
                "replace":"ঙ্ক্ষ"
            },
            {
                "find":"NGc",
                "replace":"ঞ্চ"
            },
            {
                "find":"nch",
                "replace":"ঞ্ছ"
            },
            {
                "find":"njh",
                "replace":"ঞ্ঝ"
            },
            {
                "find":"ngh",
                "replace":"ঙ্ঘ"
            },
            {
                "find":"Ngk",
                "replace":"ঙ্ক"
            },
            {
                "find":"Ngx",
                "replace":"ঙ্ষ"
            },
            {
                "find":"Ngg",
                "replace":"ঙ্গ"
            },
            {
                "find":"Ngm",
                "replace":"ঙ্ম"
            },
            {
                "find":"NGj",
                "replace":"ঞ্জ"
            },
            {
                "find":"ndh",
                "replace":"ন্ধ"
            },
            {
                "find":"nTh",
                "replace":"ন্ঠ"
            },
            {
                "find":"NTh",
                "replace":"ণ্ঠ"
            },
            {
                "find":"nth",
                "replace":"ন্থ"
            },
            {
                "find":"nkh",
                "replace":"ঙ্খ"
            },
            {
                "find":"ngo",
                "replace":"ঙ্গ"
            },
            {
                "find":"nga",
                "replace":"ঙ্গা"
            },
            {
                "find":"ngi",
                "replace":"ঙ্গি"
            },
            {
                "find":"ngI",
                "replace":"ঙ্গী"
            },
            {
                "find":"ngu",
                "replace":"ঙ্গু"
            },
            {
                "find":"ngU",
                "replace":"ঙ্গূ"
            },
            {
                "find":"nge",
                "replace":"ঙ্গে"
            },
            {
                "find":"ngO",
                "replace":"ঙ্গো"
            },
            {
                "find":"NDh",
                "replace":"ণ্ঢ"
            },
            {
                "find":"nsh",
                "replace":"নশ"
            },
            {
                "find":"Ngr",
                "replace":"ঙর"
            },
            {
                "find":"NGr",
                "replace":"ঞর"
            },
            {
                "find":"ngr",
                "replace":"ংর"
            },
            {
                "find":"nj",
                "replace":"ঞ্জ"
            },
            {
                "find":"Ng",
                "replace":"ঙ"
            },
            {
                "find":"NG",
                "replace":"ঞ"
            },
            {
                "find":"nk",
                "replace":"ঙ্ক"
            },
            {
                "find":"ng",
                "replace":"ং"
            },
            {
                "find":"nn",
                "replace":"ন্ন"
            },
            {
                "find":"NN",
                "replace":"ণ্ণ"
            },
            {
                "find":"Nn",
                "replace":"ণ্ন"
            },
            {
                "find":"nm",
                "replace":"ন্ম"
            },
            {
                "find":"Nm",
                "replace":"ণ্ম"
            },
            {
                "find":"nd",
                "replace":"ন্দ"
            },
            {
                "find":"nT",
                "replace":"ন্ট"
            },
            {
                "find":"NT",
                "replace":"ণ্ট"
            },
            {
                "find":"nD",
                "replace":"ন্ড"
            },
            {
                "find":"ND",
                "replace":"ণ্ড"
            },
            {
                "find":"nt",
                "replace":"ন্ত"
            },
            {
                "find":"ns",
                "replace":"ন্স"
            },
            {
                "find":"nc",
                "replace":"ঞ্চ"
            },
            {
                "find":"n",
                "replace":"ন"
            },
            {
                "find":"N",
                "replace":"ণ"
            },
            {
                "find":"OI`",
                "replace":"ৈ"
            },
            {
                "find":"OU`",
                "replace":"ৌ"
            },
            {
                "find":"O`",
                "replace":"ো"
            },
            {
                "find":"OI",
                "replace":"ৈ",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            }
                        ],
                        "replace":"ঐ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"ঐ"
                    }
                ]
            },
            {
                "find":"OU",
                "replace":"ৌ",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            }
                        ],
                        "replace":"ঔ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"ঔ"
                    }
                ]
            },
            {
                "find":"O",
                "replace":"ো",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            }
                        ],
                        "replace":"ও"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"ও"
                    }
                ]
            },
            {
                "find":"phl",
                "replace":"ফ্ল"
            },
            {
                "find":"pT",
                "replace":"প্ট"
            },
            {
                "find":"pt",
                "replace":"প্ত"
            },
            {
                "find":"pn",
                "replace":"প্ন"
            },
            {
                "find":"pp",
                "replace":"প্প"
            },
            {
                "find":"pl",
                "replace":"প্ল"
            },
            {
                "find":"ps",
                "replace":"প্স"
            },
            {
                "find":"ph",
                "replace":"ফ"
            },
            {
                "find":"fl",
                "replace":"ফ্ল"
            },
            {
                "find":"f",
                "replace":"ফ"
            },
            {
                "find":"p",
                "replace":"প"
            },
            {
                "find":"rri`",
                "replace":"ৃ"
            },
            {
                "find":"rri",
                "replace":"ৃ",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            }
                        ],
                        "replace":"ঋ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"ঋ"
                    }
                ]
            },
            {
                "find":"rrZ",
                "replace":"রর‍্য"
            },
            {
                "find":"rry",
                "replace":"রর‍্য"
            },
            {
                "find":"rZ",
                "replace":"র‍্য",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"r"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"y"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"w"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"x"
                            }
                        ],
                        "replace":"্র্য"
                    }
                ]
            },
            {
                "find":"ry",
                "replace":"র‍্য",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"r"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"y"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"w"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"x"
                            }
                        ],
                        "replace":"্র্য"
                    }
                ]
            },
            {
                "find":"rr",
                "replace":"রর",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!vowel"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"r"
                            },
                            {
                                "type":"suffix",
                                "scope":"!punctuation"
                            }
                        ],
                        "replace":"র্"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"r"
                            }
                        ],
                        "replace":"্রর"
                    }
                ]
            },
            {
                "find":"Rg",
                "replace":"ড়্গ"
            },
            {
                "find":"Rh",
                "replace":"ঢ়"
            },
            {
                "find":"R",
                "replace":"ড়"
            },
            {
                "find":"r",
                "replace":"র",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"r"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"y"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"w"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"x"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"Z"
                            }
                        ],
                        "replace":"্র"
                    }
                ]
            },
            {
                "find":"shch",
                "replace":"শ্ছ"
            },
            {
                "find":"ShTh",
                "replace":"ষ্ঠ"
            },
            {
                "find":"Shph",
                "replace":"ষ্ফ"
            },
            {
                "find":"Sch",
                "replace":"শ্ছ"
            },
            {
                "find":"skl",
                "replace":"স্ক্ল"
            },
            {
                "find":"skh",
                "replace":"স্খ"
            },
            {
                "find":"sth",
                "replace":"স্থ"
            },
            {
                "find":"sph",
                "replace":"স্ফ"
            },
            {
                "find":"shc",
                "replace":"শ্চ"
            },
            {
                "find":"sht",
                "replace":"শ্ত"
            },
            {
                "find":"shn",
                "replace":"শ্ন"
            },
            {
                "find":"shm",
                "replace":"শ্ম"
            },
            {
                "find":"shl",
                "replace":"শ্ল"
            },
            {
                "find":"Shk",
                "replace":"ষ্ক"
            },
            {
                "find":"ShT",
                "replace":"ষ্ট"
            },
            {
                "find":"ShN",
                "replace":"ষ্ণ"
            },
            {
                "find":"Shp",
                "replace":"ষ্প"
            },
            {
                "find":"Shf",
                "replace":"ষ্ফ"
            },
            {
                "find":"Shm",
                "replace":"ষ্ম"
            },
            {
                "find":"spl",
                "replace":"স্প্ল"
            },
            {
                "find":"sk",
                "replace":"স্ক"
            },
            {
                "find":"Sc",
                "replace":"শ্চ"
            },
            {
                "find":"sT",
                "replace":"স্ট"
            },
            {
                "find":"st",
                "replace":"স্ত"
            },
            {
                "find":"sn",
                "replace":"স্ন"
            },
            {
                "find":"sp",
                "replace":"স্প"
            },
            {
                "find":"sf",
                "replace":"স্ফ"
            },
            {
                "find":"sm",
                "replace":"স্ম"
            },
            {
                "find":"sl",
                "replace":"স্ল"
            },
            {
                "find":"sh",
                "replace":"শ"
            },
            {
                "find":"Sc",
                "replace":"শ্চ"
            },
            {
                "find":"St",
                "replace":"শ্ত"
            },
            {
                "find":"Sn",
                "replace":"শ্ন"
            },
            {
                "find":"Sm",
                "replace":"শ্ম"
            },
            {
                "find":"Sl",
                "replace":"শ্ল"
            },
            {
                "find":"Sh",
                "replace":"ষ"
            },
            {
                "find":"s",
                "replace":"স"
            },
            {
                "find":"S",
                "replace":"শ"
            },
            {
                "find":"oo`",
                "replace":"ু"
            },
            {
                "find":"oo",
                "replace":"ু",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"উ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"উ"
                    }
                ]
            },
            {
                "find":"o`",
                "replace":""
            },
            {
                "find":"oZ",
                "replace":"অ্য"
            },
            {
                "find":"o",
                "replace":"",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"vowel"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"o"
                            }
                        ],
                        "replace":"ও"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"vowel"
                            },
                            {
                                "type":"prefix",
                                "scope":"exact",
                                "value":"o"
                            }
                        ],
                        "replace":"অ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"অ"
                    }
                ]
            },
            {
                "find":"tth",
                "replace":"ত্থ"
            },
            {
                "find":"t``",
                "replace":"ৎ"
            },
            {
                "find":"TT",
                "replace":"ট্ট"
            },
            {
                "find":"Tm",
                "replace":"ট্ম"
            },
            {
                "find":"Th",
                "replace":"ঠ"
            },
            {
                "find":"tn",
                "replace":"ত্ন"
            },
            {
                "find":"tm",
                "replace":"ত্ম"
            },
            {
                "find":"th",
                "replace":"থ"
            },
            {
                "find":"tt",
                "replace":"ত্ত"
            },
            {
                "find":"T",
                "replace":"ট"
            },
            {
                "find":"t",
                "replace":"ত"
            },
            {
                "find":"aZ",
                "replace":"অ্যা"
            },
            {
                "find":"AZ",
                "replace":"অ্যা"
            },
            {
                "find":"a`",
                "replace":"া"
            },
            {
                "find":"A`",
                "replace":"া"
            },
            {
                "find":"a",
                "replace":"া",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"আ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!exact",
                                "value":"a"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"য়া"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"exact",
                                "value":"a"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"আ"
                    }
                ]
            },
            {
                "find":"i`",
                "replace":"ি"
            },
            {
                "find":"i",
                "replace":"ি",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ই"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ই"
                    }
                ]
            },
            {
                "find":"I`",
                "replace":"ী"
            },
            {
                "find":"I",
                "replace":"ী",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঈ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঈ"
                    }
                ]
            },
            {
                "find":"u`",
                "replace":"ু"
            },
            {
                "find":"u",
                "replace":"ু",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"উ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"উ"
                    }
                ]
            },
            {
                "find":"U`",
                "replace":"ূ"
            },
            {
                "find":"U",
                "replace":"ূ",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঊ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঊ"
                    }
                ]
            },
            {
                "find":"ee`",
                "replace":"ী"
            },
            {
                "find":"ee",
                "replace":"ী",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঈ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"ঈ"
                    }
                ]
            },
            {
                "find":"e`",
                "replace":"ে"
            },
            {
                "find":"e",
                "replace":"ে",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"এ"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"!exact",
                                "value":"`"
                            }
                        ],
                        "replace":"এ"
                    }
                ]
            },
            {
                "find":"z",
                "replace":"য"
            },
            {
                "find":"Z",
                "replace":"্য"
            },
            {
                "find":"y",
                "replace":"্য",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"!consonant"
                            },
                            {
                                "type":"prefix",
                                "scope":"!punctuation"
                            }
                        ],
                        "replace":"য়"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"ইয়"
                    }
                ]
            },
            {
                "find":"Y",
                "replace":"য়"
            },
            {
                "find":"q",
                "replace":"ক"
            },
            {
                "find":"w",
                "replace":"ও",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            },
                            {
                                "type":"suffix",
                                "scope":"vowel"
                            }
                        ],
                        "replace":"ওয়"
                    },
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"consonant"
                            }
                        ],
                        "replace":"্ব"
                    }
                ]
            },
            {
                "find":"x",
                "replace":"ক্স",
                "rules":
                [
                    {
                        "matches":
                        [
                            {
                                "type":"prefix",
                                "scope":"punctuation"
                            }
                        ],
                        "replace":"এক্স"
                    }
                ]
            },
            {
                "find":":`",
                "replace":":"
            },
            {
                "find":":",
                "replace":"ঃ"
            },
            {
                "find":"^`",
                "replace":"^"
            },
            {
                "find":"^",
                "replace":"ঁ"
            },
            {
                "find":",,",
                "replace":"্‌"
            },
            {
                "find":",",
                "replace":","
            },
            {
                "find":"\$",
                "replace":"৳"
            },
            {
                "find":"`",
                "replace":""
            }
        ],
        "vowel":"aeiou",
        "consonant":"bcdfghjklmnpqrstvwxyz",
        "casesensitive":"oiudgjnrstyz"
    }''';
