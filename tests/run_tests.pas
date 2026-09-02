program run_tests;

{$mode objfpc}{$H+}{$modeswitch unicodestrings}
{$codepage utf8}

uses
  classes, sysutils, clsEnglishToBangla;

var
  Parser: TEnglishToBangla;
  Fails: Integer;

procedure Check(const Input: String; const Expected: UnicodeString);
var
  ResultString: UnicodeString;
begin
  ResultString := Parser.Convert(Input);
  if ResultString = Expected then
  begin
    writeln('PASS: ' + Input + ' -> ' + UTF8Encode(Expected));
  end
  else
  begin
    writeln('FAIL: ' + Input + ' -> ' + UTF8Encode(ResultString) + ' (Expected: ' + UTF8Encode(Expected) + ')');
    Inc(Fails);
  end;
end;

begin
  Fails := 0;
  Parser := TEnglishToBangla.Create;

  Parser.AutoCorrectEnabled := False;

  writeln('Running core unit tests...');

  // Basic Vowels
  Check('a', 'আ');
  Check('e', 'এ');
  Check('i', 'ই');
  Check('O', 'ও'); // phonetic 'o' is aw, 'O' is 'o'
  Check('u', 'উ');

  // Consonants
  Check('k', 'ক');
  Check('kh', 'খ');
  Check('g', 'গ');
  Check('gh', 'ঘ');
  Check('c', 'চ');
  Check('ch', 'ছ');
  Check('j', 'জ');
  Check('jh', 'ঝ');
  Check('T', 'ট');
  Check('Th', 'ঠ');
  Check('D', 'ড');
  Check('Dh', 'ঢ');
  Check('t', 'ত');
  Check('th', 'থ');
  Check('d', 'দ');
  Check('dh', 'ধ');
  Check('n', 'ন');
  Check('p', 'প');
  Check('ph', 'ফ');
  Check('b', 'ব');
  Check('bh', 'ভ');
  Check('m', 'ম');
  Check('r', 'র');
  Check('l', 'ল');
  Check('s', 'স');
  Check('sh', 'শ');
  Check('h', 'হ');

  // Kar (Vowel signs)
  Check('ka', 'কা');
  Check('ki', 'কি');
  Check('ku', 'কু');
  Check('ke', 'কে');
  Check('kO', 'কো');

  // Fala
  Check('kya', 'ক্যা'); // jofala + a

  // Conjuncts
  Check('kkO', 'ক্কো');
  Check('kTa', 'ক্টা');
  Check('nda', 'ন্দা');
  Check('shikkha', 'শিক্ষা');

  // Words
  Check('amra', 'আম্রা');
  Check('bangla', 'বাংলা');
  Check('kotha', 'কথা');

  // ZWNJ / ZWJ? Just check basic mapping for now if unsure of syntax.
  Check('k`', 'ক'); // example backtick

  Parser.Free;

  if Fails > 0 then
  begin
    writeln(IntToStr(Fails) + ' tests failed!');
    Halt(1);
  end
  else
  begin
    writeln('All tests passed!');
    Halt(0);
  end;
end.
