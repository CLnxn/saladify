import Joi from "joi";

export type TStandardPayload = {};

export const TStandardPayloadValidator = Joi.object<TStandardPayload>();

export type TResultPayload = { result: boolean };

export const TResultPayloadValidator = Joi.object<TResultPayload>({
  result: Joi.boolean(),
});

export type TGetUsernamePayload = { username: string };

export const TGetUsernamePayloadValidator = Joi.object<TGetUsernamePayload>({
  username: Joi.string(),
});

export type TUpdateImageResponseBody = { href: string };

export const UpdateImageResponseBodyValidator =
  Joi.object<TUpdateImageResponseBody>({
    href: Joi.string().allow(""),
  });

// profile
export type TProfileBody = {
  display_name: string;
  bio: string;
  picture: string;
  following: number | null;
  followers: number | null;
  is_private: boolean;
};

export const TProfileBodyValidator = Joi.object<TProfileBody>({
  display_name: Joi.string().min(0).required(),
  bio: Joi.string().allow(null).min(0),
  picture: Joi.string().min(0),
  following: Joi.number().optional(),
  followers: Joi.number().optional(),
  is_private: Joi.boolean(),
}).unknown();

// link
export type TLink = {
  id: number;
  user_id: number;
  next_id: number | null;
  title: string | null;
  href: string;
  description: string | null;
  img_src: string | null;
};

export const TLinkBodyValidator = Joi.object<{ links: TLink[] }>({
  links: Joi.array<TLink[]>()
    .items(
      Joi.object({
        id: Joi.number(),
        user_id: Joi.number().required(),
        next_id: Joi.number().allow(null).optional(),
        href: Joi.string().min(0).required(),
        title: Joi.string().min(0).allow(null).optional(),
        description: Joi.string().min(0).allow(null).optional(),
        img_src: Joi.string().allow(null).optional(),
      }),
    )
    .min(0),
});
