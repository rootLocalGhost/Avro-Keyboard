{
  =============================================================================
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.
  =============================================================================
}


{ COMPLETE TRANSFERING! }

unit BanglaChars;

interface

const
  {$REGION 'Unusual Bangla Characters'}
  {
    '==============================================================
    '==============================================================
    'Unusual Bangla Characters
  }

  {$REGION 'Vowels'}
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'Vowels
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  b_Vocalic_L: UnicodeChar  = #$98C;
  b_Vocalic_LL: UnicodeChar = #$9E1;
  b_Vocalic_RR: UnicodeChar = #$9E0;
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    ' End Vowels
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  {$ENDREGION}
  {$REGION 'Vowels Signs (Kar/Matra)'}
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'Vowels Signs (Kar/Matra)
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  b_Vocalic_RR_Kar: UnicodeChar = #$9C4;
  b_Vocalic_L_Kar: UnicodeChar  = #$9E2;
  b_Vocalic_LL_Kar: UnicodeChar = #$9E3;
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    ' End Vowels Signs (Kar/Matra)
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  {$ENDREGION}
  {$REGION 'Signs'}
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'Signs
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  b_Nukta: UnicodeChar      = #$9BC;
  b_Avagraha: UnicodeChar   = #$9BD;
  b_LengthMark: UnicodeChar = #$9D7;
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'End Signs
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  {$ENDREGION}
  {$REGION 'Additional'}
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'Additional
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  b_RupeeMark: UnicodeChar                             = #$9F2;
  b_CurrencyNumerator1: UnicodeChar                    = #$9F4;
  b_CurrencyNumerator2: UnicodeChar                    = #$9F5;
  b_CurrencyNumerator3: UnicodeChar                    = #$9F6;
  b_CurrencyNumerator4: UnicodeChar                    = #$9F7;
  b_CurrencyNumerator1LessThanDenominator: UnicodeChar = #$9F8;
  b_CurrencyDenominator16: UnicodeChar                 = #$9F9;
  b_CurrencyEsshar: UnicodeChar                        = #$9FA;
  { '''''''''''''''''''''''''''''''''''''''''''''''''''
    'End Additional
    ''''''''''''''''''''''''''''''''''''''''''''''''''' }
  {$ENDREGION}
  { 'End Unusual Bangla Characters
    '==============================================================
    '============================================================== }
  {$ENDREGION}
  {$REGION 'Bangla Numbers'}
  { '==============================================================
    'Bangla Numbers
    '============================================================== }
  b_0: UnicodeChar = #$9E6;
  b_1: UnicodeChar = #$9E7;
  b_2: UnicodeChar = #$9E8;
  b_3: UnicodeChar = #$9E9;
  b_4: UnicodeChar = #$9EA;
  b_5: UnicodeChar = #$9EB;
  b_6: UnicodeChar = #$9EC;
  b_7: UnicodeChar = #$9ED;
  b_8: UnicodeChar = #$9EE;
  b_9: UnicodeChar = #$9EF;
  { '==============================================================
    'End Bangla Numbers
    '============================================================== }
  {$ENDREGION}
  {$REGION 'Bangla Vowels and Kars'}
  { '==============================================================
    'Bangla Vowels and Kars
    '============================================================== }
  b_A: UnicodeChar      = #$985;
  b_AA: UnicodeChar     = #$986;
  b_AAkar: UnicodeChar  = #$9BE;
  b_I: UnicodeChar      = #$987;
  b_II: UnicodeChar     = #$988;
  b_IIkar: UnicodeChar  = #$9C0;
  b_Ikar: UnicodeChar   = #$9BF;
  b_U: UnicodeChar      = #$989;
  b_Ukar: UnicodeChar   = #$9C1;
  b_UU: UnicodeChar     = #$98A;
  b_UUkar: UnicodeChar  = #$9C2;
  b_RRI: UnicodeChar    = #$98B;
  b_RRIkar: UnicodeChar = #$9C3;
  b_E: UnicodeChar      = #$98F;
  b_Ekar: UnicodeChar   = #$9C7;
  b_O: UnicodeChar      = #$993;
  b_OI: UnicodeChar     = #$990;
  b_OIkar: UnicodeChar  = #$9C8;
  b_Okar: UnicodeChar   = #$9CB;
  b_OU: UnicodeChar     = #$994;
  b_OUkar: UnicodeChar  = #$9CC;
  { '==============================================================
    'End Bangla Vowels and Kars
    '============================================================== }
  {$ENDREGION}
  {$REGION 'Bangla Consonents'}
  { '==============================================================
    'Bangla Consonents
    '============================================================== }
  b_Anushar: UnicodeChar   = #$982;
  b_B: UnicodeChar         = #$9AC;
  b_Bh: UnicodeChar        = #$9AD;
  b_Bisharga: UnicodeChar  = #$983;
  b_C: UnicodeChar         = #$99A;
  b_CH: UnicodeChar        = #$99B;
  b_Chandra: UnicodeChar   = #$981;
  b_D: UnicodeChar         = #$9A6;
  b_Dd: UnicodeChar        = #$9A1;
  b_Ddh: UnicodeChar       = #$9A2;
  b_Dh: UnicodeChar        = #$9A7;
  b_G: UnicodeChar         = #$997;
  b_GH: UnicodeChar        = #$998;
  b_H: UnicodeChar         = #$9B9;
  b_J: UnicodeChar         = #$99C;
  b_JH: UnicodeChar        = #$99D;
  b_K: UnicodeChar         = #$995;
  b_KH: UnicodeChar        = #$996;
  b_L: UnicodeChar         = #$9B2;
  b_M: UnicodeChar         = #$9AE;
  b_N: UnicodeChar         = #$9A8;
  b_NGA: UnicodeChar       = #$999;
  b_Nn: UnicodeChar        = #$9A3;
  b_NYA: UnicodeChar       = #$99E;
  b_P: UnicodeChar         = #$9AA;
  b_Ph: UnicodeChar        = #$9AB;
  b_R: UnicodeChar         = #$9B0;
  b_Rr: UnicodeChar        = #$9DC;
  b_Rrh: UnicodeChar       = #$9DD;
  b_S: UnicodeChar         = #$9B8;
  b_Sh: UnicodeChar        = #$9B6;
  b_Ss: UnicodeChar        = #$9B7;
  b_T: UnicodeChar         = #$9A4;
  b_Th: UnicodeChar        = #$9A5;
  b_Tt: UnicodeChar        = #$99F;
  b_Tth: UnicodeChar       = #$9A0;
  b_Y: UnicodeChar         = #$9DF;
  b_Z: UnicodeChar         = #$9AF;
  AssamRa: UnicodeChar     = #$9F0;
  AssamVa: UnicodeChar     = #$9F1;
  b_Khandatta: UnicodeChar = #$9CE;
  { '==============================================================
    'End Bangla Consonents
    '============================================================== }
  {$ENDREGION}
  {$REGION 'Bangla Others'}
  { '==============================================================
    'Bangla Others
    '============================================================== }
  b_Dari: UnicodeChar    = #$964;
  b_Hasanta: UnicodeChar = #$9CD;
  b_Taka: UnicodeChar    = #$9F3;
  ZWJ: UnicodeChar       = #$200D;
  ZWNJ: UnicodeChar      = #$200C;
  { '==============================================================
    'End Bangla Others
    '============================================================== }
  {$ENDREGION}
function IsVowel(const strX: string): Boolean;
function IsPureConsonent(const strX: string): Boolean;
function IsKar(const strX: string): Boolean;

implementation

{$HINTS Off}

function IsVowel(const strX: string): Boolean;
var
  WC: Char;
begin

  Result := false;

  WC := strX[1];

  if (WC = b_A) or (WC = b_AA) or (WC = b_AAkar) or (WC = b_I) or (WC = b_II) or (WC = b_IIkar) or (WC = b_Ikar) or (WC = b_U) or (WC = b_Ukar) or
    (WC = b_UU) or (WC = b_UUkar) or (WC = b_RRI) or (WC = b_RRIkar) or (WC = b_E) or (WC = b_Ekar) or (WC = b_OI) or (WC = b_OIkar) or (WC = b_O) or
    (WC = b_Okar) or (WC = b_OU) or (WC = b_OUkar) or (WC = b_Vocalic_L) or (WC = b_Vocalic_LL) or (WC = b_Vocalic_RR) or (WC = b_Vocalic_RR_Kar) or
    (WC = b_Vocalic_L_Kar) or (WC = b_Vocalic_LL_Kar) then
    Result := True
  else
    Result := false;

end;
{$HINTS On}
{$HINTS Off}

function IsPureConsonent(const strX: string): Boolean;
var
  WC: Char;
begin

  Result := false;

  WC := strX[1];

  if (WC = b_B) or (WC = b_Bh) or (WC = b_C) or (WC = b_CH) or (WC = b_D) or (WC = b_Dd) or (WC = b_Ddh) or (WC = b_Dh) or (WC = b_G) or (WC = b_GH) or
    (WC = b_H) or (WC = b_J) or (WC = b_JH) or (WC = b_K) or (WC = b_KH) or (WC = b_L) or (WC = b_M) or (WC = b_N) or (WC = b_NGA) or (WC = b_Nn) or
    (WC = b_NYA) or (WC = b_P) or (WC = b_Ph) or (WC = b_R) or (WC = b_Rr) or (WC = b_Rrh) or (WC = b_S) or (WC = b_Sh) or (WC = b_Ss) or (WC = b_T) or
    (WC = b_Th) or (WC = b_Tt) or (WC = b_Tth) or (WC = b_Z) or (WC = b_Y) or (WC = b_Khandatta) or (WC = AssamRa) or (WC = AssamVa) then
    Result := True
  else
    Result := false;
end;
{$HINTS On}
{$HINTS Off}

function IsKar(const strX: string): Boolean;
var
  WC: Char;
begin

  Result := false;

  WC := strX[1];

  if (WC = b_AAkar) or (WC = b_IIkar) or (WC = b_Ikar) or (WC = b_Ukar) or (WC = b_UUkar) or (WC = b_RRIkar) or (WC = b_Ekar) or (WC = b_OIkar) or
    (WC = b_OUkar) or (WC = b_Vocalic_RR_Kar) or (WC = b_Vocalic_L_Kar) or (WC = b_Vocalic_LL_Kar) then
    Result := True
  else
    Result := false;

end;
{$HINTS On}

end.
