# Project01 Taiwanese ID Checker
# Hayden Chang B11030202

.data
	# messages
	msg_welcome:  .asciiz  "Please input an identification number:\n"
	msg_legal:    .asciiz  "\nThe number is legal.\n"
	msg_illegal:  .asciiz  "\nThe number is illegal.\n"
	
	# ID Literal
	# A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
	literal_digit_0: .byte 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3
	literal_digit_1: .byte 0, 1, 2, 3, 4, 5, 6, 7, 4, 8, 9, 0, 1, 2, 5, 3, 4, 5, 6, 7, 8, 9, 2, 0, 1, 3
	
	# variables
	literal: .word 0
	# number: .space 9 # 9 digits * 4 bytes = 36 bytes
	number: .word 9 9 9 9 9 9 9 9 9
	sum: .word 0

.text
main:
	# print welcome message
	la	$a0, msg_welcome
	li	$v0, 4 # print message
	syscall
	
	# input ID literal
	li	$v0, 12 # input char
	syscall
	la	$t0, ($v0)
	sub $t0, $t0, 'A'
	sw 	$t0, literal
	
	# input ID numbers	
	la	$t0, ($zero) # set index to 0
	input_numbers:
		li	$v0, 12 # input char
		syscall
		la $t1, ($v0)
		subi $t2, $t1, '0'
		sb $t2,	number($t0)
		addi $t0, $t0, 1
		ble	$t0, 8, input_numbers # go back to input numbers if index <= 8
		
	# caculate sum
	la $t2, ($zero)
	lbu $t3, literal
	
	# digit 0 (a0 * 1)
	lbu $t4, literal_digit_0($t3)
	add $t2, $t2, $t4
	
	# digit 1 (a1 * 9)
	lbu $t4, literal_digit_1($t3)
	mul $t4, $t4, 9
	add $t2, $t2, $t4
	
	# digit 2 ~ 9 (a[i] * (8 - i))
	la	$t0, ($zero) # set index to 0
	li	$t1, 8
	calc_numbers:
		lb $t4, number($t0)
		mul $t4, $t4, $t1
		
		add $t2, $t2, $t4
		addi $t0, $t0, 1
		subi $t1, $t1, 1
		ble	$t0, 7, calc_numbers # go back to calc numbers if index <= 7
	
	sb $t2, sum # store sum value
	
	# caculate check sum
	li $t5, 8
	lb $t4, number($t5)
	add $t2, $t2, $t4
	
	# (sum + a[9]) / 10
	div	$t2, $t2, 10
	# get modulus
	mfhi $t3
	
	beqz $t3, legal
	bnez $t3, illegal
	
legal:
	# print legal message
	la	$a0, msg_legal
	li	$v0, 4 # print message
	syscall
	li	$v0, 10 # exit
	syscall
	
illegal:
	# print illegal message
	la	$a0, msg_illegal
	li	$v0, 4 # print message
	syscall
	li	$v0, 10 # exit
	syscall