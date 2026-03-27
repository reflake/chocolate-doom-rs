#include "m_random.h"

random_t random, pred_random;

void InitRandom(void)
{
	random = CreateRandom();
	pred_random = CreateRandom();
}

int M_Random(void)
{
	return NextRandom(&random);
}

int P_Random(void)
{
	return NextRandom(&pred_random);
}

int P_SubRandom(void)
{
	return NextSubRandom(&pred_random);
}

void M_ClearRandom(void)
{
	ClearRandom(&random);
	ClearRandom(&pred_random);
}