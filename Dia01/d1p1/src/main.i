        DO ,1 <- #255 $ #255
        DO WRITE IN ,1

        PLEASE NOTE num = 0
        PLEASE NOTE i = 0
        PLEASE NOTE for char in file:
        PLEASE NOTE mult = 100
        PLEASE NOTE while char != '\n':
        DO NOTE mult /= 10
        DO NOTE ,1[i] + = char as num * mult
        DO NOTE i +  + 
        DO NOTE i = .1 |||| mult = .2

        DO .1 <- #0
        DO .2 <- #1
        DO .4 <- #0
        DO .6 <- #0
        DO .7 <- #32767
(3)     PLEASE DON'T GIVE UP
        DO READ OUT #254
(1)     DO STASH .1
        DO STASH .2
        DO .2 <- .1
        DO .1 <- #10
        DO (1010) NEXT
        DO .1 <- .3
        DO (2001) NEXT
        DO .1 <- .3
        DO (2000) NEXT
        PLEASE RETRIEVE .1
        PLEASE RETRIEVE .2
        DO (2) NEXT
(4)     DO FORGET #1
(2)     DO FORGET .3
        DO RESUME #1
        DO COME FROM (4)
        DO (1000) NEXT
        DO .1 <- .3
        PLEASE NOTE THIS IS THE SCOPE OF THE LOOP, .1 is the index
        DO NOT READ OUT .1
        DO STASH .1
        DO STASH .2
        DO .2 <- #1
        DO (1010) NEXT
        DO .1 <- .3
        DO .2 <- #4
        DO (1030) NEXT
        DO .1 <- .3
        DO .2 <- #1
        DO (1000) NEXT
        PLEASE DO STASH .3
        PLEASE NOTE THIS IS THE DECODING
        DO .2 <- .4
        DO .1 <- ,1 SUB .3
        DO (1000) NEXT
        DO .4 <- .3
        DO .1 <- .3
        DO .2 <- #48
        DO (1010) NEXT
        DO .1 <- .3
        DO .2 <- #100
        DO (1030) NEXT
        DO .5 <- .3
        PLEASE DO RETRIEVE .3
        DO .1 <- .3
        DO .2 <- #1
        DO (1000) NEXT
        PLEASE DO STASH .3
        DO .2 <- .4
        DO .1 <- ,1 SUB .3
        DO (1000) NEXT
        DO .3 <- .3 ~ #255
        DO .4 <- .3
        DO .1 <- .3
        DO .2 <- #48
        PLEASE DO (1010) NEXT
        DO .1 <- .3
        DO .2 <- #10
        DO (1030) NEXT
        PLEASE DO .1 <- .5
        DO .2 <- .3
        DO (1000) NEXT
        DO .5 <- .3
        PLEASE DO RETRIEVE .3
        DO .1 <- .3
        DO .2 <- #1
        PLEASE DO (1000) NEXT
        DO .2 <- .4
        DO .1 <- ,1 SUB .3
        DO (1000) NEXT
        DO .3 <- .3 ~ #255
        DO .1 <- .3
        PLEASE DO .2 <- #48
        PLEASE DO (1010) NEXT
        DO .1 <- .5
        DO .2 <- .3
        DO (1000) NEXT
        PLEASE DO NOT READ OUT .3
        DO .5 <- .3
        PLEASE NOTE .5 IS THE PARSED NUMBER
        DO .1 <- .7
        DO .2 <- .5
        PLEASE DO (2002) NEXT
        DO (5) NEXT
        DO NOT READ OUT .7 + .5 + #65535
        DO .1 <- .6
        DO (1020) NEXT
        DO .6 <- .1
        DO NOTE IF .3 IS #1
        DO (6) NEXT
(5)     DO (7) NEXT
(8)     DO FORGET #1
(7)     DO FORGET .3
        DO RESUME #1
(6)     DO FORGET #1
        DO COME FROM (8)
        DO .7 <- .5
        DO .5 <- #0
        DO .4 <- #10
        PLEASE DO RETRIEVE .1
        PLEASE DO RETRIEVE .2
        DO COME FROM (3)
        PLEASE DO (1) NEXT
        PLEASE DO READ OUT .6
        PLEASE DO GIVE UP


(2000)  DO NOTE INVERT BIT
        DO .3 <- '?"#1 $ .1"' ~ #1
        DO RESUME #1

(2001)  DO NOTE GREATER THAN 0
        DO .3 <- !1 ~ .1' ~ #1
        DO RESUME #1

(2002)  DO NOTE .2 greater than .1
        DO (1010) NEXT
        DO .3 <- .3 ~ #32768
        DO RESUME #1

