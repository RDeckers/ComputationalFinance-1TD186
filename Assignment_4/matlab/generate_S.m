function [ S ] = generate_S( N, B, K )
deltaS = (B-K)/(N+1);
S = K+deltaS:deltaS:B-deltaS; %gives N steps, starting from s = deltaS

end

