# IAM Flow

- unified entry point for authorization request. it accepts various identity
  types. to make it easier, we will pick phone number as the example.
- the request creates AuthorizationPhoneNumber. we have
  a verification system in here (PhoneNumberVerification is a mixin?)
- upon confirmed, it looks up if the phone number is associated to any user.
  if it's associated to a user, the user is logging in. if there's no users
  associated with the phone number, we'll create a user and associate the
  number with it (atomic operation). any of the cases creates a terminal.


PhoneNumberVerification has an event port for successful confirmation.
We connect this port to a function which checks for the associated user, and
then create if it's not exist. This function can be generalized.

## Functions

Return entity ID by one of its unique attributes. If no entity is having
the attribute, create a new instance of the entity, associate the attribute and
return the ID. All in one atomic operation.

Create entity e with unique attribute x on conflict return e.id

EntityIDByUniqueAttribute(create: bool)

## Mixins

### Phone number verification

Provides verification flow for phone numbers.

#### Parameters

`allowed_methods`
: Options: `sms`, `call`

`routes`
: Map of method and region (country code) with the registered providers.
