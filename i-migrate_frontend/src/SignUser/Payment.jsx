"use client";

import { Button } from "@/components/ui/button";

export function PaymentOptions({ amount }) {
  const handleEthereumPayment = () => {
    alert(`Processing Ethereum payment of $${amount}`);
    // In a real app, this would connect to a wallet and process the payment
  };

  const handleStripePayment = () => {
    alert(`Processing credit card payment of $${amount}`);
    // In a real app, this would redirect to Stripe or process the payment
  };

  return (
    <div className="space-y-4">
      <p className="text-center mb-4">Select how you want to pay ${amount}</p>

      <Button
        onClick={handleEthereumPayment}
        className="w-full mb-4 bg-purple-600 hover:bg-purple-700"
      >
        Pay with Ethereum
      </Button>

      <Button onClick={handleStripePayment} className="w-full">
        Pay with Credit Card
      </Button>
    </div>
  );
}
