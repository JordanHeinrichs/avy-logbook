export const load = async ({ params }) => {
  return {
    tripId: Number(params.tripId),
  };
};
