    SECTION "Init", ROM0[$100]
Init:
    nop
    jp Start

    SECTION "Main", ROM0[$150]
Start:
    ld a, $e4
    ld [$FF47], a

    ld	a,$91
	ld	[$FF40],a ; enable lcd

    ld b, 16
    ld HL, $8000
    ld DE, Tile

LoadTile:
    ld a, [DE]
    ld [HL], a
    inc DE
    inc HL
    dec b
    ld a, 0
    cp b
    jp NZ, LoadTile


    ld bc, $400
    ld HL, $9800
    xor a
ClearTileMap:
    ld [HL+], a
    dec bc
    cp b
    jp NZ, ClearTileMap
    cp c
    jp NZ, ClearTileMap

Loop:
    jp Loop

Tile:
    DB $0f,$00,$2f,$24,$2f,$24,$0f,$00
    DB $f0,$0f,$f2,$4f,$fc,$3f,$f0,$0f
