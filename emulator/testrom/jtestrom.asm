    SECTION "Init", ROM0[$100]
Init:
    nop
    jp Start

    SECTION "Main", ROM0[$150]
Start:
    di
    ld a, $e4
    ld [$FF47], a

    ld A, $10                   ; Set joypad to direction only
    ld [$FF00], a

    ld	a,$93
	ld	[$FF40],a ; enable lcd

    ld a, 32
LoadTiles:
    ld HL, $8010
    ld DE, TileData+$10
LoadTilesInner:
    ld [$ff80], a               ; Backup a
    call LoadTile
    ld a, [$ff80]
    dec a
    jr NZ, LoadTilesInner
    jp ClearTileMap

LoadTile:
    ld b, 16
LoadTileInner:
    call WaitTilVBlank
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

    call WaitTilVBlank
    ld HL, $8000
    ld DE, TileData
    call LoadTile

SetFloorTiles:
    ld BC, 512
    ld HL, $9A00
SetFloorTilesInner:
    call WaitTilVBlank
    nop
    nop
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
    ld a, $10                    ; sprite x
    ld [HL+], a
    ld a, 11
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
    ld A, [$FE00]               ; sprite 0 y loc
    ld B, 136                   ; floor height to fake collision
    cp b
    jr NC, SkipDec
    inc A
    ld [$FE00], A
    jp ScrollLoop
SkipDec:
    call JoypadMovement
    jp ScrollLoop

JoypadMovement:
    ld A, [$FF00]
    bit 0, A
    jr Z, MoveRight
    ld A, [$FF00]
    bit 1, A
    jr Z, MoveLeft
    jr DoneMove
MoveRight:
    ld A, [$FE01]               ; sprite 0 x loc
    inc A
    ld [$FE01], A
    jr DoneMove
MoveLeft:
    ld A, [$FE01]               ; sprite 0 x loc
    dec A
    ld [$FE01], A
DoneMove:
    ret



WaitTilVBlank:
WaitTilVBlankInner:
    ld A, [$FF44]
    sub 144
    jp C, WaitTilVBlankInner
    ld A, [$FF44]
    sub 148
    jp NC, WaitTilVBlankInner
WaitTilVBlankDone:
    ret


    SECTION "TileData", ROM0[$2000]
TileData:
    incbin "jnumbers"
