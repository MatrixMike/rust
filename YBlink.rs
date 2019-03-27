#define WDTPW	0x5A00
#define WDTHOLD	0x0080

typedef unsigned char uint8_t;
typedef unsigned short int uint16_t;
typedef uint16_t size_t;

extern uint8_t P1OUT;
extern uint8_t P1DIR;
extern uint16_t WDTCTL;

int main(void)
{
	size_t i;

	WDTCTL = WDTPW | WDTHOLD;
	P1DIR = 0x40;

	while (1) {
		P1OUT ^= 0x40;
		for (i = 0; i < 5000; i++);
	}
}
