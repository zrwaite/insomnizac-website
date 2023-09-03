import type { HomeData } from '$lib/types'

export async function load({ data }: { data: HomeData }): Promise<HomeData> {
	return data
}
