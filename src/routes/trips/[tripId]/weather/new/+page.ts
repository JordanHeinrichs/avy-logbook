export const load = async ({ params, url }) => {
  return {
    tripId: Number(params.tripId),
    isPlan: !!url.searchParams.get("plan"),
    wizard: !!url.searchParams.get("wizard"),
  };
};
