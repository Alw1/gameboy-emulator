; Hello World Program for Game Boy

; Define the ROM header
org $100
jp start

; Define the start of the program
start:
    ; Initialize the display
    xor a
    ld [rstat], a ; Disable interrupts
    ld a, $01
    ld [rdisp], a ; Set display control register
    ld a, $01
    ld [rscm], a ; Enable screen display

    ; Initialize the video RAM
    ld hl, $9800 ; Set HL to the start of the background map
    ld a, $00
    ld (hl), a ; Clear the background map

    ; Display "HELLO"
    ld hl, $9800 ; Start of background map
    ld a, $48    ; ASCII for 'H'
    call print_char
    ld a, $45    ; ASCII for 'E'
    call print_char
    ld a, $4C    ; ASCII for 'L'
    call print_char
    ld a, $4C    ; ASCII for 'L'
    call print_char
    ld a, $4F    ; ASCII for 'O'
    call print_char

    ; Endless loop
    halt

