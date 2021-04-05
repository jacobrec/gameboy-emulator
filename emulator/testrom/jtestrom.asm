    SECTION "Init", ROM0[$100]
Init:
    nop
    jp Start

    SECTION "Main", ROM0[$150]
Start:
    di
    ld a, $e4
    ld [$FF47], a

    ld	a,$93
	ld	[$FF40],a ; enable lcd

    ld a, 32
LoadTiles:
    ld HL, $8000
    ld DE, TileData
LoadTilesInner:
    ld [$ff80], a
    call LoadTile
    ld a, [$ff80]
    inc HL
    inc DE
    dec a
    jr NZ, LoadTilesInner
    jp ClearTileMap

LoadTile:
    call WaitTilVBlank
    ld b, 16
LoadTileInner:
    ld a, [DE]
    ld [HL], a
    inc DE
    inc HL
    dec b
    jr NZ, LoadTileInner
    ret



ClearTileMap:
    ld BC, 1024
    ld HL, $9800
ClearTileMapInner:
    call WaitTilVBlank
    ld a, 10
    ld [HL+], a
    dec BC
    xor A
    cp b
    jp NZ, ClearTileMapInner
    cp c
    jp NZ, ClearTileMapInner

SetFloorTiles:
    ld BC, 512
    ld HL, $9A00
SetFloorTilesInner:
    call WaitTilVBlank
    ld a, 12
    ld [HL+], a
    dec BC
    xor A
    cp b
    jp NZ, SetFloorTilesInner
    cp c
    jp NZ, SetFloorTilesInner

LoadSprite1Data:
    ld HL, $FE00
    ld a, $10                    ; sprite y
    ld [HL+], a
    ld a, $8                    ; sprite x
    ld [HL+], a
    ld a, 11
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
    ld C, 2                     ; frames per line
Loop:

    ld A, [$FF44]
    cp B
    jp NZ, Loop
    dec B
    dec C
    jp NZ, Loop

Scroll:                         ; move sprite 0 down, until it aligns where platform is
    ld A, [$FE00]
    ld B, 136
    cp b
    jr NC, SkipDec
    inc A
    ld [$FE00], A
SkipDec:

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


    SECTION "TileData", ROM0[$2000]
TileData:
    incbin "jnumbers"
