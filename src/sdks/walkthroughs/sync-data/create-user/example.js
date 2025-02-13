await permit.api.users.sync({
  key: "auth0|elon",
  email: "elonmusk@tesla.com",
  first_name: "Elon",
  last_name: "Musk",
  attributes: {
    department: "marketing",
    age: 30,
    subscription: {
      tier: "pro",
      expired: false,
    },
  },
});
