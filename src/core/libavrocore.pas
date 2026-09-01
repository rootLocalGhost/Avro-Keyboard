library libavrocore;

{$mode objfpc}{$H+}{$modeswitch unicodestrings}

uses
  classes, sysutils, clsEnglishToBangla;

var
  Parser: TEnglishToBangla;

function avro_convert(input_str: PAnsiChar): PAnsiChar; cdecl;
var
  InputU, OutputU: UnicodeString;
  OutputA: AnsiString;
  Res: PAnsiChar;
begin
  if Parser = nil then
  begin
    Parser := TEnglishToBangla.Create;
    Parser.AutoCorrectEnabled := False;
  end;

  InputU := UTF8Decode(AnsiString(input_str));
  OutputU := Parser.Convert(InputU);
  OutputA := UTF8Encode(OutputU);

  Res := StrAlloc(Length(OutputA) + 1);
  StrPCopy(Res, OutputA);

  Result := Res;
end;

procedure avro_free_string(str: PAnsiChar); cdecl;
begin
  if str <> nil then
    StrDispose(str);
end;

procedure avro_init; cdecl;
begin
  if Parser = nil then
  begin
    Parser := TEnglishToBangla.Create;
    Parser.AutoCorrectEnabled := False;
  end;
end;

procedure avro_cleanup; cdecl;
begin
  if Parser <> nil then
  begin
    Parser.Free;
    Parser := nil;
  end;
end;

exports
  avro_convert,
  avro_free_string,
  avro_init,
  avro_cleanup;

begin
  Parser := nil;
end.
