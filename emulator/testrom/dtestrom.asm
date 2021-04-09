    SECTION "Main", ROM0[$0]
Start:
    ld a, $80
    ld [$FF26], a ; enable sound

    ld	a,$77
	ld	[$FF24],a ; max volume for left and right terminals

    ld	a, $FF
	ld	[$FF25],a ; enable all channel output

    ld	a, $16
	ld	[$FF10],a 

    ld	a, $40
	ld	[$FF11],a 

    ld	a, $73
	ld	[$FF12],a 

    ld	a, $00
	ld	[$FF13],a 

    ld	a, $C3
	ld	[$FF14],a 

SECTION "Entry", ROM0[$100]
    jp Start
