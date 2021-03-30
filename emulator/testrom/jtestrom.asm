    SECTION "Init", ROM0[$100]
Init:
    nop
    jp Start

    SECTION "Main", ROM0[$150]
Start:
    di
    ld a, $e4
    ld [$FF47], a

    ld	a,$91
	ld	[$FF40],a ; enable lcd

    ld b, 16
    ld HL, $8000
    ld DE, Tile
    call WaitTilVBlank
LoadTile:
    ld a, [DE]
    ld [HL], a
    inc DE
    inc HL
    dec b
    jp NZ, LoadTile

    ld b, 16
    ld HL, $8010
    ld DE, Sprite1Tile
    call WaitTilVBlank
LoadSprite:
    ld a, [DE]
    ld [HL], a
    inc DE
    inc HL
    dec b
    jp NZ, LoadSprite


    ld BC, 1024
    ld HL, $9800
ClearTileMap:
    call WaitTilVBlank
    ld a, 0
    ld [HL+], a
    dec BC
    xor A
    cp b
    jp NZ, ClearTileMap
    cp c
    jp NZ, ClearTileMap

LoadSprite1Data:
    ld HL, $FE00
    ld a, $10                    ; sprite y
    ld [HL+], a
    ld a, $8                    ; sprite x
    ld [HL+], a
    ld a, $1
    ld [HL+], a
    ld a, $0
    ld [HL+], a
    ;; Sprite 2
    ld a, $20                    ; sprite y
    ld [HL+], a
    ld a, $8                    ; sprite x
    ld [HL+], a
    ld a, $1
    ld [HL+], a
    ld a, $0
    ld [HL+], a

ScrollLoop:
    ld B, 150                   ; scroll on line 150
    ld C, 5                    ; frames per line
Loop:

    ld A, [$FF44]
    cp B
    jp NZ, Loop
    dec B
    dec C
    jp NZ, Loop
Scroll:
    ld A, [$FF42]               ; Scroll
    dec A

    ld A, [$FE01]
    inc a
    ld [$FE01], A

    ld [$FF42], A
    jp ScrollLoop


WaitTilVBlank:
WaitTilVBlankInner:
    ld A, [$FF44]
    sub 145
    jp C, WaitTilVBlankInner
    ld A, [$FF44]
    sub 149
    jp NC, WaitTilVBlankInner
WaitTilVBlankDone:
    ret

Tile:
    DB $0f,$00,$2f,$24,$2f,$24,$0f,$00
    DB $f0,$0f,$f2,$4f,$fc,$3f,$f0,$0f

Sprite1Tile:
    DB $7f,$7f,$ff,$ff,$ff,$ff,$ff,$ff
    DB $ff,$ff,$ff,$ff,$fd,$fd,$ff,$ff
