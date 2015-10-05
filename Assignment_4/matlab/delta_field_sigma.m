function [] = delta_field_sigma( N, M,  sigma_min, sigma_max, delta_sigma)
    V = compute_field_sigma(N, M,  sigma_min, sigma_max, delta_sigma);
    figure(2)
    mask = [-1 0 1]
    K=15; %strike price
    B = 2*K;
    T=0.5; %time of maturity
    deltaT = T/M;
    r=0.1; %rate of interest
    gamma = 1.0;

    S = generate_S(N, B, K);
    delta = diff(V);
    size(delta)
    size(V)
    values = V(end-N+1:end);
    sigma= sigma_min:delta_sigma:sigma_max;
    size(S)
    hold on;
    h= pcolor(sigma, S(1:end-1), delta)
    set(h,'edgecolor','none')
    hold off;
    hcb = colorbar('location','EastOutside');
    caxis([-0.2 0.2])
end

