program avro_cli;

{$mode objfpc}{$H+}{$modeswitch unicodestrings}

uses
  classes, sysutils, clsEnglishToBangla;

var
  Parser: TEnglishToBangla;
  InputWord: UnicodeString;
  OutputWord: UnicodeString;
  i: Integer;
begin
  if ParamCount = 0 then
  begin
    writeln('Usage: avro_cli <word1> [word2 ...]');
    Halt(1);
  end;

  Parser := TEnglishToBangla.Create;
  Parser.AutoCorrectEnabled := False;

  for i := 1 to ParamCount do
  begin
    InputWord := ParamStr(i);
    OutputWord := Parser.Convert(InputWord);
    writeln(UTF8Encode(OutputWord));
  end;

  Parser.Free;
end.
