.data   
    # meeages
MSG_INPUT_N:    .asciiz "Please input first positive integer n:\n"
MSG_INPUT_K:    .asciiz "Please input second positive integer k:\n"

MSG_OUTPUT_1:   .asciiz "The binomial coefficient C("
MSG_OUTPUT_2:   .asciiz ", "
MSG_OUTPUT_3:   .asciiz ") = "

MSG_INVALID:    .asciiz "Invalid value of k\n"

.text   
main:           
    # show input n message
    la      $a0,            MSG_INPUT_N
    li      $v0,            4
    syscall 

    # input n($s0)
    li      $v0,            5
    syscall 
    move    $s0,            $v0

    # show input k message
    la      $a0,            MSG_INPUT_K
    li      $v0,            4
    syscall 

    # input k($s1)
    li      $v0,            5
    syscall 
    move    $s1,            $v0

    # show invalid message if k < 0
    blt     $s1,            $zero,              showInvalidValMsg

    # show invalid message if k > n
    bgt     $s1,            $s0,                showInvalidValMsg

    # copy n($s0), k($s1) to $a0, $a1 for comb(n, k)
    move    $a0,            $s0
    move    $a1,            $s1

    # call comb
    jal     comb
    move    $t0,            $v0

    # print result
    la      $a0,            MSG_OUTPUT_1
    li      $v0,            4
    syscall 

    # print n
    move    $a0,            $s0
    li      $v0,            1
    syscall 

    la      $a0,            MSG_OUTPUT_2
    li      $v0,            4
    syscall 

    # print k
    move    $a0,            $s1
    li      $v0,            1
    syscall 

    la      $a0,            MSG_OUTPUT_3
    li      $v0,            4
    syscall 

    # print C(n, k)
    move    $a0,            $t0
    li      $v0,            1
    syscall 

    j       exit

    # binomialCoefficient
    # comb(n: $a0, k: $a1) => $v0
comb:           
    # if k != 0 and k != n jump to calc comb_calc
    bnez    $a1,            comb_k_not_zero
    j       comb_return1

comb_k_not_zero:
    bne     $a1,            $a0,                comb_calc
    j       comb_return1

    # {
comb_return1:   
    # set result to 1
    addi    $v0,            $zero,              1

    # return
    jr      $ra

    # } else {
comb_calc:      
    # step 1
    # binomialCoefficient(n - 1, k - 1)
    move    $t0,            $a0
    move    $t1,            $a1

    addi    $a0,            $a0,                -1
    addi    $a1,            $a1,                -1

    # adjust stack for 7 items
    addi    $sp,            $sp,                -28
    # save registers values to the stack
    sw      $ra,            24($sp)
    sw      $a1,            20($sp)
    sw      $a0,            16($sp)
    sw      $t3,            12($sp)
    sw      $t2,            8($sp)
    sw      $t1,            4($sp)
    sw      $t0,            0($sp)

    # call function
    jal     comb
    # restore registers values from the stack
    lw      $ra,            24($sp)
    lw      $a1,            20($sp)
    lw      $a0,            16($sp)
    lw      $t3,            12($sp)
    lw      $t2,            8($sp)
    lw      $t1,            4($sp)
    lw      $t0,            0($sp)
    addi    $sp,            $sp,                28

    move    $a0,            $t0
    move    $a1,            $t1

    move    $t2,            $v0

    # step 2
    # binomialCoefficient(n - 1, k)
    move    $t0,            $a0
    move    $t1,            $a1

    addi    $a0,            $a0,                -1

    # adjust stack for 7 items
    addi    $sp,            $sp,                -28
    # save registers values to the stack
    sw      $ra,            24($sp)
    sw      $a1,            20($sp)
    sw      $a0,            16($sp)
    sw      $t3,            12($sp)
    sw      $t2,            8($sp)
    sw      $t1,            4($sp)
    sw      $t0,            0($sp)

    # call function
    jal     comb
    # restore registers values from the stack
    lw      $ra,            24($sp)
    lw      $a1,            20($sp)
    lw      $a0,            16($sp)
    lw      $t3,            12($sp)
    lw      $t2,            8($sp)
    lw      $t1,            4($sp)
    lw      $t0,            0($sp)
    addi    $sp,            $sp,                28

    move    $a0,            $t0
    move    $a1,            $t1

    move    $t3,            $v0

    # add step 2 result and step 1 result
    add     $v0,            $t2,                $t3

    # return
    jr      $ra

    # }

    # showInvalidValMsg()
showInvalidValMsg:
    la      $a0,            MSG_INVALID
    li      $v0,            4
    syscall 

    j       exit

    # exit()
exit:           
    li      $v0,            10
    syscall 