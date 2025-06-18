"use client";

import { useState } from "react";
import { UserForm } from "./PaymentForm";
import { PaymentOptions } from "./Payment";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function PaymentFlow() {
  const [step, setStep] = useState("form");
  const [userData, setUserData] = useState(null);

  const handleFormSubmit = (data) => {
    setUserData(data);
    setStep("payment");
  };

  return (
    <Card className="w-[500px] m-auto mt-[10%]">
      <CardHeader>
        <CardTitle>
          {step === "form" ? "Enter Your Details" : "Choose Payment Method"}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {step === "form" ? (
          <UserForm onSubmit={handleFormSubmit} />
        ) : (
          <PaymentOptions amount={userData?.amount || 0} />
        )}
      </CardContent>
    </Card>
  );
}
