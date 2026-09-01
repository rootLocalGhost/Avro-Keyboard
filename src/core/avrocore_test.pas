program avrocore_test;

{$mode objfpc}{$H+}{$modeswitch unicodestrings}

uses
  classes, sysutils, clsEnglishToBangla;

var
  Parser: TEnglishToBangla;
  Out1, Out2: UnicodeString;
begin
  Parser := TEnglishToBangla.Create;
  Parser.AutoCorrectEnabled := False;
  Out1 := Parser.Convert('amra');
  Out2 := Parser.Convert('bangla');
  writeln(UTF8Encode(Out1));
  writeln(UTF8Encode(Out2));
  Parser.Free;
end.
