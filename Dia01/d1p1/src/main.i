        DO ,1 <- #255 $ #255
        DO ,2 <- #255 $ #255
        DO WRITE IN ,2

        PLEASE NOTE num = 0
        PLEASE NOTE i = 0
        PLEASE NOTE for char in file:
        PLEASE NOTE mult = 100
        PLEASE NOTE while char != '\n':
        DO NOTE mult /= 10
        DO NOTE ,1[i] + = char as num * mult
        DO NOTE i +  + 
        DO NOTE i = .1 |||| mult = .2

        DO .1 <- #10
        DO .2 <- #1
(3)     PLEASE DON'T GIVE UP
        DO READ OUT #254
(1)     DO (2001) NEXT
        DO STASH .1
        DO .1 <- .3
        DO (2000) NEXT
        PLEASE RETRIEVE .1
        DO (2) NEXT
(4)     DO FORGET #1
(2)     DO FORGET .3
        DO RESUME #1
        DO COME FROM (4)
        DO (1010) NEXT
        DO .1 <- .3
        DO READ OUT .1
        DO COME FROM (3)
        PLEASE DO (1) NEXT
        DO NOT READ OUT #255
        PLEASE DO GIVE UP


(2000)  DO NOTE INVERT BIT
        DO .3 <- '?"#1 $ .1"' ~ #1
        DO RESUME #1

(2001)  DO NOTE GREATER THAN 0
        DO .3 <- !1 ~ .1' ~ #1
        DO RESUME #1

