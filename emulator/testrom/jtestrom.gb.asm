    SECTION "Main", ROM0[$0]
Start:
    ld sp, $fffe
    xor a
    ld hl, $9fff

    ld a, $e4
    ld [$FF47], a

    ld	a,90
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

    jp Start

Tile:
    db $0F, $00, $0F, $00, $0F, $00, $0F, $00, $0F, $FF, $0F, $FF, $0F, $FF, $0F, $FF
