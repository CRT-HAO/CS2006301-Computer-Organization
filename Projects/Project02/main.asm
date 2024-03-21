.data
	# meeages
	MSG_INPUT_N: .asciiz "Please input first positive integer n:\n"
	MSG_INPUT_K: .asciiz "Please input second positive integer k:\n"
	
	MSG_OUTPUT_1: .asciiz "The binomial coefficient C("
	MSG_OUTPUT_2: .asciiz ", "
	MSG_OUTPUT_3: .asciiz ") = "
	
	# variables
	k: .word 0
	n: .word 0
	
.text
main:
	# show input n message
	la $a0, MSG_INPUT_N
	li $v0, 4
	syscall
	
	# show input k message
	la $a0, MSG_INPUT_K
	li $v0, 4
	syscall
	
	j exit
	
# binomialCoefficient(n: $a0, k: $a1) => $v0
binomialCoefficient:
	# adjust stack for 3 items
	addi $sp, $sp, -12
  # save registers values to the stack
  sw $ra, 8($sp)
  sw $a1, 4($sp)
  sw $a0, 0($sp)
	
exit:
	li $v0, 10
	syscall