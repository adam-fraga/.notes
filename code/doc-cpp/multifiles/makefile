math: math_stff.o math_utils.o
		g++ math_stff.o math_utils.o -o math

math_stff.o: math_stff.cpp
		g++ -c math_stff.cpp

math_utils.o: math_utils.cpp
		g++ -c math_utils.cpp

clean:
		rm * .o math

# "make" lance le makefile et génère ici le fichier executable math auquel est associé nos .o
# on associe nos .o a chacun des fichier .cpp correspondant
# A chaque modification d'un .cpp apres un make le fichier math est régénéré et mis a jour
# "make clean" éfface nos binaires .o et math
