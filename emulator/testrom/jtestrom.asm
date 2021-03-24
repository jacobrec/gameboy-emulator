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

    ld B, 150
WaitTilVBlank:
    ld A, [$FF44]
    sub 150
    jp NZ, WaitTilVBlank

LoadTile:
    ld a, [DE]
    ld [HL], a
    inc DE
    inc HL
    dec b
    jp NZ, LoadTile


    ld BC, $400
    ld HL, $9800
    xor a
ClearTileMap:
    ld [HL+], a
    dec BC
    cp b
    jp NZ, ClearTileMap
    cp c
    jp NZ, ClearTileMap

ScrollLoop:
    ld B, 150
    ld C, 5                    ; frames per line
Loop:

    ld A, [$FF44]
    cp B
    jp NZ, Loop
    dec B
    dec C
    jp NZ, Loop
Scroll:
    ld A, [$FF42]
    dec A
    ld [$FF42], A
    jp ScrollLoop

Tile:
    DB $0f,$00,$2f,$24,$2f,$24,$0f,$00
    DB $f0,$0f,$f2,$4f,$fc,$3f,$f0,$0f
