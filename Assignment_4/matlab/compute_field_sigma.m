 function [V] = compute_field_sigma( N,M, sigma_min, sigma_max, delta_sigma )
K=15; %strike price
B = 2*K;
T=0.5; %time of maturity
deltaT = T/M;
r=0.1; %rate of interest
sigma= sigma_min:delta_sigma:sigma_max;
gamma = 1.0;

S = generate_S(N, B, K);
V = [];
figure(7)
hold on
for s = sigma
    s;
    A = generate_A_matrix(N, s, r, B,K, gamma);
    F = 3*speye(N)+2*deltaT*A;
    %F = speye(N)+deltaT*A;
    v = transpose(S - K); %value at time t=T (initial condition), discounted to be in t=0 'dollars'.
    v_old = v;
    for k = 1:M 
            v_new = F\(4*v-v_old);
            v_old = v;
            v = v_new;
            %v = F\v;    
    end
    V = [V [v]];
    if s == sigma_min
        plot(S,v)
    end
    if abs(s-(sigma_min + 0.5*(sigma_max-sigma_min))) < delta_sigma/2
        plot(S,v)
    end
    s;
    v(end/2);
end
plot(S,v)
figure(1)
V;
%colorDepth = 1000;
%colormap(jet(colorDepth));
values = sigma;
hold on;
h= pcolor(values, S, V);
set(h,'edgecolor','none') ;
[C,hfigc] = contour(values, S, V);
set(hfigc, ...
    'LineWidth',1.0, ...
    'Color', [1 1 1]);
hold off;
hcb = colorbar('location','EastOutside');
end
