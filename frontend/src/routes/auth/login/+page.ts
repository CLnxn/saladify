import type { PageLoad } from "./$types";

/**
 * validates and prepares the corresponding page data
 * @param param0
 * @returns an object to be pointed to by 'data' variable in +page.svelte
 */
export const load: PageLoad = async ({ url, data, route, fetch, params }) => {
  return {
    reset: false,
  };
};
