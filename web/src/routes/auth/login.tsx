import { NavLink } from "react-router";
import { Logo } from "../../components/logo";
import { Button } from "../../components/ui/button";
import { Input } from "../../components/ui/input";
import { useForm, type SubmitHandler } from "react-hook-form";
import { login } from "../../lib/api/auth";
import { useAuthStore } from "../../lib/state/auth";
import { isError } from "../../lib/api/error";
import { useState } from "react";
import { Loader } from "../../components/ui/loader";
import { CircleAlert } from "lucide-react";

export interface Inputs {
  email: string;
  password: string;
}

export function Login() {
  const updateUser = useAuthStore(state => state.updateUser);
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<Inputs>();
  const [error, setError] = useState<string | null>(null);

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    await login(data).then(result => {
      if (isError(result)) {
        setError(result.error);
        return;
      }
      updateUser(result);
      window.location.href = "/";
    })

  };

  return (
    <div className="h-full w-full flex items-center justify-center flex-col">
      <Logo className="h-10 w-10 text-pink-500 mb-6" />
      <h1 className="font-display text-2xl font-bold mb-8">Welcome Back!</h1>
      <form
        className="max-w-2xs w-full flex flex-col mb-10"
        onSubmit={handleSubmit(onSubmit)}
      >
        <Input
          placeholder="Email"
          type="text"
          className="w-full"
          {...register("email", {
            required: { value: true, message: "Required." },
            pattern: {
              value:
                /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|.(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/,
              message: "Invalid email.",
            },
          })}
        />
        {errors.email ? <p className="text-red-500 text-sm font-medium">{errors.email.message}</p> : null}
        <div className="h-2" />
        <Input
          placeholder="Password"
          type="password"
          className="w-full"
          {...register("password", {
            required: { value: true, message: "Requred." },
            minLength: { value: 8, message: "Must be at least 8 characters long." },
            maxLength: { value: 72, message: "Must be at most 72 characters long." },
          })}
        />
        {errors.password ? <p className="text-red-500 text-sm font-medium">{errors.password.message}</p> : null}
        <Button intent="primary" className="mt-6" disabled={isSubmitting} type="submit">
          {isSubmitting ? <Loader className="text-pink-950 h-5 w-5" /> : null}
          Log In
        </Button>
        {error ? (
          <p className="text-red-500 font-medium font-display text-center text-sm">{error}</p>
        ) : null}
        <div className="flex items-start gap-2 px-2 py-2 rounded-lg border border-pink-900/20 mt-4 text-pink-900 font-display text-sm">
          <CircleAlert className="h-8 w-8" />
          Please allow third-party cookies after you log in.
        </div>
      </form>
      <p className="text-black/50 font-display">
        Don't have an account?{" "}
        <NavLink to="/auth/signup">
          Sign Up
        </NavLink>
        .
      </p>
    </div>
  );
}
