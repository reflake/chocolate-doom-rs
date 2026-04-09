#include "m_random.h"

extern random_t pred_random;

void InitRandom(void)
{
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