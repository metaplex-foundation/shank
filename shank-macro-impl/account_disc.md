## Problem

- anchor includes an 8-byte discriminator prefix when serializing accounts
- this prefix is derived from the account name and is used to identify accounts on chain
- it is also used in anchors pre-check code to verify that a passed account is correct
- it is somewhat brittle in a way opaque to the user since changing the name (in the code) of
  the account will change its discriminator, thus previous accounts stop working

We want to add a similar feature to shank in a way that makes it easy to use, has less magic
and can be used for existing accounts that use other ways to _discriminate_ their type.

## Requirements

- statically analyzable in order to include with the IDL definition at compile
- easy to use and ideally mostly opaque to the user
- support existing account setups, i.e. ones that use _keys_ to explicitly discriminate
  accounts
  - if we don't support this then existing programs cannot use that feature as they cannot
    easily change the account structure to include a discriminator prefix

## TokenMetadata Metaplex

```rs
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, ShankAccount)]
pub struct UseAuthorityRecord {
    #[discriminator(Key::UseAuthorityRecord)]
    pub key: Key,          //1
    pub allowed_uses: u64, //8
    pub bump: u8,
}
```

```rs
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, ShankAccount)]
pub struct CollectionAuthorityRecord {
    #[discriminator(Key::CollectionAuthorityRecord)]
    pub key: Key, //1
    pub bump: u8, //1
}
```

- would need to be able to _interpret_ enums to resolve them to the serizialized discriminator
- depends on user not reordering the enum variants


## Anchor-like Explicit/Implicit

```rs
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, ShankAccount)]
pub struct AnchorLike {
    #[discriminator(1, 2, 3, 4, 5, 6, 7, 8)]
    _disc: [u8; 8],
    pub bump: u8, //1
}
```

- simplest solution, just require the attribute to be on top of a bytes array prop
- should imple the below so user doesn't have to to set `_disc` to value in the attribute

```rs
impl AnchorLike {
  fn shank_discriminator() -> [u8; 8] {
    [1, 2, 3, 4, 5, 6, 7, 8]
  }
}
```

```rs
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, ShankAccount)]
pub struct AnchorLike {
    #[discriminator]
    _disc: [u8; 8],
    pub allowed_uses: u64
    pub bump: u8,
}
```

- without a value the discriminator would be derived from the account name and we could auto
  impl the below

```rs
impl AnchorLike {
  fn shank_discriminator() -> [u8; 8] {
    derive_discriminator_from("AnchorLike")
  }
}
```

## Other Helpers

- in both the above cases we could impl a helper method to serialize the account auto-inserting
  the discriminator

```rs
pub struct AnchorLikeArgs {
    pub allowed_uses: u64
    pub bump: u8,
}

impl AnchorLike {
  pub new(args: AnchorLikeArgs) -> Self {
    let AnchorLikeArgs {
      allowed_uses,
      bump
    } = args;
    Self {
      _disc: Self::shank_discriminator(),
      allowed_uses,
      bump
    }
  }

  pub shank_serialize<'a>(&self, info: &mut &mut AccountInfo) -> Result<RefMut<&'a mut [u8]>, ProgramError> {
    self.serialize(info.try_borrow_mut_data()?.as_mut())
  }
}
```
