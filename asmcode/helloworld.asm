LDA #$01
bird:
STA $03,X
LDA $D012
BEQ bird
