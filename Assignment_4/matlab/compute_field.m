function [V] = compute_field( N,M )
K=15; %strike price
B = 2*K;
T=0.5; %time of maturity
deltaT = T/M;
r=0.1; %rate of interest
sigma=0.5; %volatility
gamma = 1.0;

S = generate_S(N, B, K)
A = generate_A_matrix(N, sigma, r, B,K, gamma)
F = 3*speye(N)+2*deltaT*A;
%F = speye(N)+deltaT*A;
v = transpose(S - K) %value at time t=T (initial condition), discounted to be in t=0 'dollars'.
v_old = v;
V = [v];
for k = 1:M
        v_new = F\(4*v-v_old);
        v_old = v;
        v = v_new;
        %v = F\v;
        V = [V [v]];
end
%colorDepth = 1000;
%colormap(jet(colorDepth));
values = V(end-N+1:end);
T = fliplr(0:deltaT:T);
hold on;
h= pcolor(T, S, V)
set(h,'edgecolor','none')
[C,hfigc] = contour(T, S, V);
set(hfigc, ...
    'LineWidth',1.0, ...
    'Color', [1 1 1]);
hold off;
hcb = colorbar('location','EastOutside');
end
