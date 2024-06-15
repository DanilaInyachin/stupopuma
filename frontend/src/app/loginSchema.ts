import i18n from "i18next";
import { literal, object, string } from "zod";

// 👇 Login Schema with Zod
const loginSchema = object({
    email: string().min(1, i18n.t('Email is required')).email('Email is invalid'),
    password: string()
      .min(1, 'Password is required')
      .min(6, 'Password must be more than 6 characters')
      .max(32, 'Password must be less than 32 characters'),
    persistUser: literal(true).optional(),
  });

export default loginSchema;